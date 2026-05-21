pub mod file;

use std::fmt::Error;
use sysinfo::System;
use qtbridge::{qobject_impl, QApp};

use iced_x86::{
    Decoder,
    DecoderOptions,
    Formatter,
    Instruction,
    NasmFormatter,
    IntelFormatter,
};

#[derive(Default)]
pub struct Backend {}

#[qobject_impl(Singleton)]
impl Backend {

    #[qslot]
    fn load_file(&self, path: String) {
        let path = path.replace("file:///","").replace("file://","");

        let file_data: Vec<u8> = match std::fs::read(&path) {
            Ok(data) => {
                data
            }
            Err(e) => {
                eprintln!("Failed to read file: {e} (path: {path})");
                return;
            }
        };

        let file_type = file::get_type(&file_data);
        println!("File type found {:?}", file_type);
        let file_arch = file::get_arch(&file_data);
        println!("File {:?}", file_arch)

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