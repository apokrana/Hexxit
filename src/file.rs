
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


// typedef struct _IMAGE_DATA_DIRECTORY {
//     DWORD VirtualAddress;
//     DWORD Size;
// } IMAGE_DATA_DIRECTORY, *PIMAGE_DATA_DIRECTORY;

// layout fields in order like C
#[repr(C)] 
pub struct ImageDataDirectory {
    pub virtual_address: u32,
    pub size: u32,
}

// typedef struct _IMAGE_OPTIONAL_HEADER64 {
//   WORD                 Magic;
//   BYTE                 MajorLinkerVersion;
//   BYTE                 MinorLinkerVersion;
//   DWORD                SizeOfCode;
//   DWORD                SizeOfInitializedData;
//   DWORD                SizeOfUninitializedData;
//   DWORD                AddressOfEntryPoint;
//   DWORD                BaseOfCode;
//   ULONGLONG            ImageBase;
//   DWORD                SectionAlignment;
//   DWORD                FileAlignment;
//   WORD                 MajorOperatingSystemVersion;
//   WORD                 MinorOperatingSystemVersion;
//   WORD                 MajorImageVersion;
//   WORD                 MinorImageVersion;
//   WORD                 MajorSubsystemVersion;
//   WORD                 MinorSubsystemVersion;
//   DWORD                Win32VersionValue;
//   DWORD                SizeOfImage;
//   DWORD                SizeOfHeaders;
//   DWORD                CheckSum;
//   WORD                 Subsystem;
//   WORD                 DllCharacteristics;
//   ULONGLONG            SizeOfStackReserve;
//   ULONGLONG            SizeOfStackCommit;
//   ULONGLONG            SizeOfHeapReserve;
//   ULONGLONG            SizeOfHeapCommit;
//   DWORD                LoaderFlags;
//   DWORD                NumberOfRvaAndSizes;
//   IMAGE_DATA_DIRECTORY DataDirectory[IMAGE_NUMBEROF_DIRECTORY_ENTRIES];
// } IMAGE_OPTIONAL_HEADER64, *PIMAGE_OPTIONAL_HEADER64;

const IMAGE_NUMBER_OF_DIRECTORY_ENTRIES: usize = 16;
#[repr(C)] 
pub struct ImageOptionalHeaderx64 {
    pub magic: u16,
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub size_of_code: u32,
    pub size_of_initialized_data: u32,
    pub size_of_uninitialized_data: u32,
    pub address_of_entrypoint: u32,
    pub base_of_code: u32,
    pub image_base: u64,
    pub section_alignment: u32,
    pub file_alignment: u32,
    pub major_operating_system_version: u16,
    pub minor_operating_system_version: u16,
    pub major_image_version: u16,
    pub minor_image_version: u16,
    pub major_subsystem_version: u16,
    pub minor_subsystem_version: u16,
    pub win32_version_value: u32,
    pub size_of_image: u32,
    pub size_of_headers: u32,
    pub checksum: u32,
    pub subsystem: u16,
    pub dll_characteristics: u16,
    pub size_of_stack_reserve: u64,
    pub size_of_stack_commit: u64,
    pub size_of_heap_reserve: u64,
    pub size_of_heap_commit: u64,
    pub loader_flags: u32,
    pub number_of_rva_and_sizes: u32,
    pub data_directory: [ImageDataDirectory; IMAGE_NUMBER_OF_DIRECTORY_ENTRIES]
}

// typedef struct _IMAGE_OPTIONAL_HEADER {
//   WORD                 Magic;
//   BYTE                 MajorLinkerVersion;
//   BYTE                 MinorLinkerVersion;
//   DWORD                SizeOfCode;
//   DWORD                SizeOfInitializedData;
//   DWORD                SizeOfUninitializedData;
//   DWORD                AddressOfEntryPoint;
//   DWORD                BaseOfCode;
//   DWORD                BaseOfData;
//   DWORD                ImageBase;
//   DWORD                SectionAlignment;
//   DWORD                FileAlignment;
//   WORD                 MajorOperatingSystemVersion;
//   WORD                 MinorOperatingSystemVersion;
//   WORD                 MajorImageVersion;
//   WORD                 MinorImageVersion;
//   WORD                 MajorSubsystemVersion;
//   WORD                 MinorSubsystemVersion;
//   DWORD                Win32VersionValue;
//   DWORD                SizeOfImage;
//   DWORD                SizeOfHeaders;
//   DWORD                CheckSum;
//   WORD                 Subsystem;
//   WORD                 DllCharacteristics;
//   DWORD                SizeOfStackReserve;
//   DWORD                SizeOfStackCommit;
//   DWORD                SizeOfHeapReserve;
//   DWORD                SizeOfHeapCommit;
//   DWORD                LoaderFlags;
//   DWORD                NumberOfRvaAndSizes;
//   IMAGE_DATA_DIRECTORY DataDirectory[IMAGE_NUMBEROF_DIRECTORY_ENTRIES];
// } IMAGE_OPTIONAL_HEADER32, *PIMAGE_OPTIONAL_HEADER32;

#[repr(C)] 
pub struct ImageOptionalHeaderx86 {
    pub magic: u16,
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub size_of_code: u32,
    pub size_of_initialized_data: u32,
    pub size_of_uninitialized_data: u32,
    pub address_of_entry_point: u32,
    pub base_of_code: u32,
    pub base_of_data: u32,
    pub image_base: u32,
    pub section_alignment: u32,
    pub file_alignment: u32,
    pub major_operating_system_version: u16,
    pub minor_operating_system_version: u16,
    pub major_image_version: u16,
    pub minor_image_version: u16,
    pub major_subsystem_version: u16,
    pub minor_subsystem_version: u16,
    pub win32_version_value: u32,
    pub size_of_image: u32,
    pub size_of_headers: u32,
    pub checksum: u32,
    pub subsystem: u16,
    pub dll_characteristics: u16,
    pub size_of_stack_reserve: u32,
    pub size_of_stack_commit: u32,
    pub size_of_heap_reserve: u32,
    pub size_of_heap_commit: u32,
    pub loader_flags: u32,
    pub number_of_rva_and_sizes: u32,
    pub data_directory: [ImageDataDirectory; IMAGE_NUMBER_OF_DIRECTORY_ENTRIES]
}

// typedef struct _IMAGE_FILE_HEADER {
//   WORD  Machine;
//   WORD  NumberOfSections;
//   DWORD TimeDateStamp;
//   DWORD PointerToSymbolTable;
//   DWORD NumberOfSymbols;
//   WORD  SizeOfOptionalHeader;
//   WORD  Characteristics;
// } IMAGE_FILE_HEADER, *PIMAGE_FILE_HEADER;
#[repr(C)] 
pub struct ImageFileHeader {
    pub machine: u16,
    pub number_of_sections: u16,
    pub time_date_stamp: u32,
    pub pointer_to_symbol_table: u32,
    pub number_of_symbols: u32,
    pub size_of_optional_header: u16,
    pub characteristics: u16, 
}

// typedef struct _IMAGE_SECTION_HEADER {
//   BYTE  Name[IMAGE_SIZEOF_SHORT_NAME];
//   union {
//     DWORD PhysicalAddress;
//     DWORD VirtualSize;
//   } Misc;
//   DWORD VirtualAddress;
//   DWORD SizeOfRawData;
//   DWORD PointerToRawData;
//   DWORD PointerToRelocations;
//   DWORD PointerToLinenumbers;
//   WORD  NumberOfRelocations;
//   WORD  NumberOfLinenumbers;
//   DWORD Characteristics;
// } IMAGE_SECTION_HEADER, *PIMAGE_SECTION_HEADER;

#[repr(C)]
pub struct ImageSectionHeader {
    pub name: [u8; 8],

    pub misc: u32, // union (VirtualSize / PhysicalAddress)

    pub virtual_address: u32,
    pub size_of_raw_data: u32,
    pub pointer_to_raw_data: u32,
    pub pointer_to_relocations: u32,
    pub pointer_to_linenumbers: u32,

    pub number_of_relocations: u16,
    pub number_of_linenumbers: u16,

    pub characteristics: u32,
}

// typedef struct _IMAGE_NT_HEADERS64 {
//     DWORD                   Signature;
//     IMAGE_FILE_HEADER       FileHeader;
//     IMAGE_OPTIONAL_HEADER64 OptionalHeader;
// } IMAGE_NT_HEADERS64, *PIMAGE_NT_HEADERS64;

// typedef struct _IMAGE_NT_HEADERS {
//   DWORD                   Signature;
//   IMAGE_FILE_HEADER       FileHeader;
//   IMAGE_OPTIONAL_HEADER32 OptionalHeader;
// } IMAGE_NT_HEADERS32, *PIMAGE_NT_HEADERS32;
pub struct ImageNtHeadersx64 {
    pub signature: u32,
    pub file_header: ImageFileHeader,
    pub optional_header: ImageOptionalHeaderx64
}

pub struct ImageNtHeadersx86 {
    pub signature: u32,
    pub file_header: ImageFileHeader,
    pub optional_header: ImageOptionalHeaderx86
}





// check if ms-dos // pe 
pub fn get_type(data: &[u8]) -> FileType {
    if data.get(0) == Some(&0x4D) && data.get(1) == Some(&0x5A) {
        println!("Valid MZ header");

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


