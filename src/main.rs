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
}

impl Default for Backend {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            tx,
            rx: Mutex::new(rx),
        }
    }
}

enum LoadResult {
    Success { name: String, size: u32, magic: u32 },
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

    let size = file_data.len() as u32;

    let magic = if file_data.len() >= 2 {
        u16::from_le_bytes([file_data[0], file_data[1]]) as u32
    } else {
        0
    };

    thread::sleep(time::Duration::from_secs(5));
    
    return LoadResult::Success { name, size, magic };
}

#[qobject_impl(Singleton)]
impl Backend {
    #[qsignal]
    fn file_loaded_status(&self, success: bool) {}

    #[qsignal]
    fn file_load_start(&self, start: bool) {}

    #[qsignal]
    fn file_info(&self, name: String, size: u32, magic: u32) {}

    #[qslot]
    fn poll_results(&self) {
        let rx = self.rx.lock().unwrap();
        while let Ok(result) = rx.try_recv() {
            match result {
                LoadResult::Success { name, size, magic } => {
                    self.file_info(name, size, magic);
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