use crate::winternals::*;

use zerocopy::{TryFromBytes, LittleEndian, U16, U32};

#[derive(Debug)]
pub enum FileType {
    Unknown,
    MSDOS,
    PE,
}
#[derive(Debug)]
pub enum Arch {
    X86,
    X64,
    Unknown,
}

// check if ms-dos // pe 
pub fn get_type(data: &[u8]) -> FileType {
    if data.len() <= size_of::<ImageDosHeader>() {
        return FileType::Unknown;
    }

    let dos_bytes = &data[..size_of::<ImageDosHeader>()];
    
    let dos_header = match ImageDosHeader::try_ref_from_bytes(dos_bytes) {
        Ok(header) => header,
        Err(e) => {
            println!("Failed to parse DOS header: {:?}", e);
            return FileType::Unknown;
        }
    };

    println!("DOS header e_magic: {:X}", dos_header.e_magic);
    if dos_header.e_magic == IMAGE_DOS_SIGNATURE {
        let e_lfanew = u32::from_le_bytes([
            data[0x3C],
            data[0x3D],
            data[0x3E],
            data[0x3F],
        ]) as usize;

        if e_lfanew + 4 > data.len() {
            return FileType::MSDOS;
        }

        // check pe sig
        if &data[e_lfanew..e_lfanew + 4] == b"PE\0\0" {
            return FileType::PE;
        }

        return FileType::MSDOS;
    } 
    
    return FileType::Unknown;
}

pub fn get_arch(data: &[u8]) -> Arch {
        if data.len() < 0x40 || &data[0..2] != b"MZ" {
        return Arch::Unknown;
    }
    let pe_offset = u32::from_le_bytes([
        data[0x3C],
        data[0x3D],
        data[0x3E],
        data[0x3F],
    ]) as usize;

    // check pe sig
    if pe_offset + 6 > data.len() || &data[pe_offset..pe_offset + 4] != b"PE\0\0" {
        return Arch::Unknown;
    }

    let opt_header_offset = pe_offset + 4 + 20;

    if opt_header_offset + 2 > data.len() {
        return Arch::Unknown;
    }

    let magic = u16::from_le_bytes([
        data[opt_header_offset],
        data[opt_header_offset + 1],
    ]);

    match magic {
        0x10b => Arch::X86,
        0x20b => Arch::X64,
        _ => Arch::Unknown,
    }
}


