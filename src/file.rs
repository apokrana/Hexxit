use crate::winternals::*;

use zerocopy::{TryFromBytes, LittleEndian, U16, U32};

#[derive(Debug, Clone, PartialEq, Default)]
pub enum FileType {
    #[default]
    Unknown,
    MSDOS,
    PE,
}
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Arch {
    #[default]
    X86,
    X64,
    Unknown,
}

#[derive(Debug, Clone, Default)]
pub struct FileInfo {
    pub name: String,

    pub size: u64,
    pub data: Vec<u8>,

    pub arch: Arch,
    pub file_type: FileType,
    pub magic: u32,
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
        let e_lfanew = dos_header.e_lfanew.get() as usize;

        if e_lfanew + 4 > data.len() {
            return FileType::MSDOS;
        }

        let nt_headers = match ImageNtHeadersx64::try_ref_from_bytes(&data[e_lfanew..e_lfanew + size_of::<ImageNtHeadersx64>()]) {
            Ok(header) => header,
            Err(e) => {
            println!("Failed to parse NT headers: {:?}", e);
            return FileType::MSDOS;
            }
        };

        // check pe sig
        if nt_headers.signature == NT_HEADERS_PE_SIGNATURE {
            return FileType::PE;
        }

        return FileType::MSDOS;
    } 
    
    return FileType::Unknown;
}

pub fn get_arch(data: &[u8]) -> Arch {
    let Ok((dos_header, _)) = ImageDosHeader::try_ref_from_prefix(data) else {
        return Arch::Unknown;
    };

    if dos_header.e_magic.get() != IMAGE_DOS_SIGNATURE {
        return Arch::Unknown;
    }

    let pe_offset = dos_header.e_lfanew.get() as usize;

    if pe_offset + std::mem::size_of::<PeCommonHeader>() > data.len() {
        return Arch::Unknown;
    }

    let common_bytes = &data[pe_offset..pe_offset + std::mem::size_of::<PeCommonHeader>()];
    
    let Ok(common_header) = PeCommonHeader::try_ref_from_bytes(common_bytes) else {
        return Arch::Unknown;
    };

    if common_header.signature.get() != NT_HEADERS_PE_SIGNATURE {
        return Arch::Unknown;
    }

    match common_header.machine.get() {
        IMAGE_FILE_MACHINE_I386 => {
            let nt_bytes = &data[pe_offset..];
            if let Ok((nt_headers, _)) = ImageNtHeadersx86::try_ref_from_prefix(nt_bytes) {
                if nt_headers.optional_header.magic.get() == IMAGE_NT_OPTIONAL_HDR32_MAGIC {
                    return Arch::X86;
                }
            }
            Arch::Unknown
        }
        IMAGE_FILE_MACHINE_AMD64 => {
            let nt_bytes = &data[pe_offset..];
            if let Ok((nt_headers, _)) = ImageNtHeadersx64::try_ref_from_prefix(nt_bytes) {
                if nt_headers.optional_header.magic.get() == IMAGE_NT_OPTIONAL_HDR64_MAGIC {
                    return Arch::X64;
                }
            }
            Arch::Unknown
        }
        _ => Arch::Unknown,
    }
}


