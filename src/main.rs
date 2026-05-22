pub mod file;
pub mod winternals;

use std::{thread,time,fmt::Error,sync::{mpsc, Mutex}};
use sysinfo::System;
use qtbridge::{qobject, qobject_impl, QApp, qsignal};

// use iced_x86::{
//     Decoder,
//     DecoderOptions,
//     Formatter,
//     Instruction,
//     NasmFormatter,
//     IntelFormatter,
// };

pub struct Backend {
    rx: Mutex<mpsc::Receiver<LoadResult>>,
    tx: mpsc::Sender<LoadResult>,
    loaded_files: Mutex<Vec<file::File>>,
}

impl Default for Backend {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            tx,
            rx: Mutex::new(rx),
            loaded_files: Mutex::new(Vec::new()),
        }
    }
}

enum LoadResult {
    Success(file::File),
    Failure,
}

pub fn load_file_internal(path: &str) -> LoadResult {
    let path = path.replace("file:///","").replace("file://","");

    let file_data: Vec<u8> = match std::fs::read(&path) {
        Ok(data) => {
            data
        }
        Err(e) => {
            eprintln!("Failed to read file: {e} (path: {path})");
            return LoadResult::Failure;
        }
    };

    let file_type = file::get_type(&file_data);
    println!("File type found {:?}", file_type);
    let file_arch = file::get_arch(&file_data);
    println!("File {:?}", file_arch);

    if file_type == file::FileType::Unknown {
        return LoadResult::Failure;
    }

    let name = std::path::Path::new(&path)
        .file_name()
        .map(|os_str| os_str.to_string_lossy().into_owned())
        .unwrap_or_else(|| "Unknown".to_string());

    let size = file_data.len() as u64;

    let magic = if file_data.len() >= 2 {
        u16::from_le_bytes([file_data[0], file_data[1]]) as u32
    } else {
        0
    };
    
    return LoadResult::Success(file::File {
        name,
        size,
        magic,
        data: file_data,
    });
}

#[qobject_impl(Singleton)]
impl Backend {
    #[qsignal]
    fn file_loaded_status(&self, success: bool) {}

    #[qsignal]
    fn file_load_start(&self, start: bool) {}

    #[qsignal]
    fn file_info(&self, name: String, size: u64, magic: u32) {}

    #[qslot]
    fn poll_results(&self) {
        let rx = self.rx.lock().unwrap();
        while let Ok(result) = rx.try_recv() {
            match result {
                LoadResult::Success(file) => {
                    self.file_info(file.name.clone(), file.size, file.magic);
                    self.loaded_files.lock().unwrap().push(file);
                    self.file_loaded_status(true);
                }
                LoadResult::Failure => {
                    self.file_loaded_status(false);
                }
            }
        }
    }
    
    #[qslot]
    fn load_file(&self, path: String) -> bool {
        self.file_load_start(true);

        let tx = self.tx.clone();
        thread::spawn(move || {
            let result = load_file_internal(&path);
            let _ = 
            tx.send(result);
        });

        return true;
    }

    #[qsignal]
    fn hex_data(&self, rows: Vec<String>) {}

    #[qslot]
    fn get_hex_data(&self, file_index: u32, offset: u64, rows: u32) {
        let files = self.loaded_files.lock().unwrap();
        let Some(file) = files.get(file_index as usize) else { return; };

        let result: Vec<String> = file.data
            .chunks(16)
            .enumerate()
            .map(|(i, row)| {
                let addr = i * 16;
                let hex: String = row.iter()
                    .map(|b| format!("{:02X}", b))
                    .collect::<Vec<_>>()
                    .join(" ");
                let ascii: String = row.iter()
                    .map(|&b| if b.is_ascii_graphic() { b as char } else { '.' })
                    .collect();
                format!("{:08X}  {:<47}  {}", addr, hex, ascii)
            })
            .collect();

        self.hex_data(result);
    }

    #[qslot]
    fn get_processes(&self) -> Vec<String> {
        let mut system = System::new_all();

        system.refresh_all();

        let mut processes = Vec::new();

        for process in system.processes().values() {
            processes.push(format!(
                "{} ({})",
                process.name().to_string_lossy(),
                process.pid()
            ));
        }

        processes.sort();

        processes
    }

}

fn main() {
    <Backend as qtbridge::QmlRegister>::register();
    QApp::new()
        .load_qml(include_bytes!("../qml/main.qml"))
        .run();
}