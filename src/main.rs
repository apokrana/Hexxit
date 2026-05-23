pub mod file;
pub mod winternals;

use std::{thread,time,fmt::Error,fmt::Write, sync::{mpsc, Mutex}};
use sysinfo::System;
use qtbridge::{qobject, qobject_impl, QApp, qsignal};
use arboard::Clipboard;
use file::Arch;
// use iced_x86::{
//     Decoder,
//     DecoderOptions,
//     Formatter,
//     Instruction,
//     NasmFormatter,
//     IntelFormatter,
// };

enum LoadResult {
    Success(file::FileInfo),
    Failure,
}

pub fn load_file_internal(path: &str) -> LoadResult {
    let path = path.replace("file:///","").replace("file://","");

    let data: Vec<u8> = match std::fs::read(&path) {
        Ok(data) => {
            data
        }
        Err(e) => {
            eprintln!("Failed to read file: {e} (path: {path})");
            return LoadResult::Failure;
        }
    };

    let file_type = file::get_type(&data);
    println!("File type found {:?}", file_type);
    let arch = file::get_arch(&data);
    println!("File {:?}", arch);

    if file_type == file::FileType::Unknown {
        return LoadResult::Failure;
    }

    let name = std::path::Path::new(&path)
        .file_name()
        .map(|os_str| os_str.to_string_lossy().into_owned())
        .unwrap_or_else(|| "Unknown".to_string());

    let size = data.len() as u64;

    let magic = if data.len() >= 2 {
        u16::from_le_bytes([data[0], data[1]]) as u32
    } else {
        0
    };
    
    return LoadResult::Success(file::FileInfo {
        name,
        size,
        data,
        arch,
        file_type,
        magic,
    });
}

fn char_to_byte_idx(char_pos: u32, data_len: usize, row_offset: u32) -> Option<usize> {
    const ROW_LEN: u32 = 76;
    const HEX_START: u32 = 10;
    const HEX_END: u32 = 57;
    let row = (char_pos / ROW_LEN) as usize + row_offset as usize;
    let col = (char_pos % ROW_LEN).clamp(HEX_START, HEX_END - 1);
    let byte_idx = row * 16 + ((col - HEX_START) / 3) as usize;
    (byte_idx < data_len).then_some(byte_idx)
}

pub struct Backend {
    rx: Mutex<mpsc::Receiver<LoadResult>>,
    tx: mpsc::Sender<LoadResult>,
    loaded_files: Mutex<Vec<file::FileInfo>>,
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


#[qobject_impl(Singleton)]
impl Backend {

    // context bar helpers

    #[qslot]
    fn copy_raw(&self, file_index: u32, sel_start: u32, sel_end: u32, row_offset: u32) -> bool {
        let files = self.loaded_files.lock().unwrap();
        let Some(file) = files.get(file_index as usize) else { return false; };

        let start = match char_to_byte_idx(sel_start, file.data.len(), row_offset) {
            Some(b) => b,
            None => return false,
        };
        let end = match char_to_byte_idx(sel_end.saturating_sub(1), file.data.len(), row_offset) {
            Some(b) => b + 1,
            None => return false,
        };

        if start >= end { return false; }

        let output = file.data[start..end]
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<_>>()
            .join(" ");

        let mut clipboard = match Clipboard::new() {
            Ok(c) => c,
            Err(_) => return false,
        };

        match clipboard.set_text(output) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    #[qslot]
    fn copy_as_vec_cpp(&self, file_index: u32, sel_start: u32, sel_end: u32, row_offset: u32) -> bool {
        let files = self.loaded_files.lock().unwrap();
        let Some(file) = files.get(file_index as usize) else { return false; };

        let start = match char_to_byte_idx(sel_start, file.data.len(), row_offset) {
            Some(b) => b,
            None => return false,
        };
        let end = match char_to_byte_idx(sel_end.saturating_sub(1), file.data.len(), row_offset) {
            Some(b) => b + 1,
            None => return false,
        };

        if start >= end { return false; }

        let inner = file.data[start..end]
            .iter()
            .map(|b| format!("0x{:02X}", b))
            .collect::<Vec<_>>()
            .join(", ");

        let output = format!("std::vector<uint8_t> data = {{ {} }};", inner);

        let mut clipboard = match Clipboard::new() {
            Ok(c) => c,
            Err(_) => return false,
        };

        match clipboard.set_text(output) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    #[qslot]
    fn copy_as_vec_rs(&self, file_index: u32, sel_start: u32, sel_end: u32, row_offset: u32) -> bool {
        let files = self.loaded_files.lock().unwrap();
        let Some(file) = files.get(file_index as usize) else { return false; };

        let start = match char_to_byte_idx(sel_start, file.data.len(), row_offset) {
            Some(b) => b,
            None => return false,
        };
        let end = match char_to_byte_idx(sel_end.saturating_sub(1), file.data.len(), row_offset) {
            Some(b) => b + 1,
            None => return false,
        };

        let inner = file.data[start..end]
            .iter()
            .map(|b| format!("0x{:02X}", b))
            .collect::<Vec<_>>()
            .join(", ");

        let output = format!("let data: Vec<u8> = vec![{}];", inner);

        let mut clipboard = match Clipboard::new() {
            Ok(c) => c,
            Err(_) => return false,
        };

        match clipboard.set_text(output) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    #[qslot]
    fn get_file_count(&self) -> u32 {
        self.loaded_files.lock().unwrap().len() as u32
    }

    #[qslot]
    fn get_file_name(&self, index: u32) -> String {
        let files = self.loaded_files.lock().unwrap();

        files.get(index as usize)
            .map(|f| f.name.clone())
            .unwrap_or_default()
    }

    #[qslot]
    fn get_file_size(&self, index: u32) -> u64 {
        let files = self.loaded_files.lock().unwrap();

        files.get(index as usize)
            .map(|f| f.size)
            .unwrap_or(0)
    }

    #[qslot]
    fn get_file_magic(&self, index: u32) -> u32 {
        let files = self.loaded_files.lock().unwrap();

        files.get(index as usize)
            .map(|f| f.magic)
            .unwrap_or(0)
    }

    #[qsignal]
    fn file_loaded_status(&self, success: bool) {}

    #[qsignal]
    fn file_load_start(&self, start: bool) {}

    #[qsignal]
    fn file_info(&self, name: String, arch: String, size: u64, magic: u32) {}

    #[qslot]
    fn poll_results(&self) {
        let rx = self.rx.lock().unwrap();
        while let Ok(result) = rx.try_recv() {
            match result {
                LoadResult::Success(file) => {
                    let arch = match file.arch {
                        Arch::X86 => "x86".to_string(),
                        Arch::X64 => "x64".to_string(),
                        Arch::Unknown => "Unknown".to_string()
                    };
                    self.file_info(file.name.clone(), arch, file.size, file.magic);
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
    fn hex_data(&self, text: String) {}

    #[qslot]
    fn get_hex_data(&self, file_index: u32, offset: u64, rows: u32) {
        let files = self.loaded_files.lock().unwrap();
        let Some(file) = files.get(file_index as usize) else { return; };

        let row_offset = offset as usize;
        let row_count = if rows > 0 { rows as usize } else { usize::MAX };


        let result = file.data
            .chunks(16)
            .enumerate()
            .skip(row_offset)
            .take(row_count)
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
            .collect::<Vec<_>>()
            .join("\n");

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