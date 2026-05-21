use zerocopy::{TryFromBytes, KnownLayout, Immutable, LittleEndian, U16, U32, U64};


pub const IMAGE_DOS_SIGNATURE: u16 = 0x5A4D;            // MZ
pub const NT_HEADERS_PE_SIGNATURE: u32 = 0x00004550;    // PE00

pub const IMAGE_FILE_MACHINE_I386: u16 = 0x014c;        // x86
pub const IMAGE_FILE_MACHINE_AMD64: u16 = 0x8664;       // x64
pub const IMAGE_NT_OPTIONAL_HDR32_MAGIC: u16 = 0x010b;  // x86
pub const IMAGE_NT_OPTIONAL_HDR64_MAGIC: u16 = 0x020b;  // x64

#[repr(C)] 
#[derive(TryFromBytes, KnownLayout, Immutable, Debug, Clone, Copy)]
pub struct ImageDataDirectory {
    pub virtual_address: U32<LittleEndian>,
    pub size: U32<LittleEndian>,
}

const IMAGE_NUMBER_OF_DIRECTORY_ENTRIES: usize = 16;
#[repr(C)] 
#[derive(TryFromBytes, KnownLayout, Immutable, Debug, Clone, Copy)]
pub struct ImageOptionalHeaderx64 {
    pub magic: U16<LittleEndian>,
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub size_of_code: U32<LittleEndian>,
    pub size_of_initialized_data: U32<LittleEndian>,
    pub size_of_uninitialized_data: U32<LittleEndian>,
    pub address_of_entrypoint: U32<LittleEndian>,
    pub base_of_code: U32<LittleEndian>,
    pub image_base: U64<LittleEndian>,
    pub section_alignment: U32<LittleEndian>,
    pub file_alignment: U32<LittleEndian>,
    pub major_operating_system_version: U16<LittleEndian>,
    pub minor_operating_system_version: U16<LittleEndian>,
    pub major_image_version: U16<LittleEndian>,
    pub minor_image_version: U16<LittleEndian>,
    pub major_subsystem_version: U16<LittleEndian>,
    pub minor_subsystem_version: U16<LittleEndian>,
    pub win32_version_value: U32<LittleEndian>,
    pub size_of_image: U32<LittleEndian>,
    pub size_of_headers: U32<LittleEndian>,
    pub checksum: U32<LittleEndian>,
    pub subsystem: U16<LittleEndian>,
    pub dll_characteristics: U16<LittleEndian>,
    pub size_of_stack_reserve: U64<LittleEndian>,
    pub size_of_stack_commit: U64<LittleEndian>,
    pub size_of_heap_reserve: U64<LittleEndian>,
    pub size_of_heap_commit: U64<LittleEndian>,
    pub loader_flags: U32<LittleEndian>,
    pub number_of_rva_and_sizes: U32<LittleEndian>,
    pub data_directory: [ImageDataDirectory; IMAGE_NUMBER_OF_DIRECTORY_ENTRIES]
}


#[repr(C)] 
#[derive(TryFromBytes, KnownLayout, Immutable, Debug, Clone, Copy)]
pub struct ImageOptionalHeaderx86 {
    pub magic: U16<LittleEndian>,
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub size_of_code: U32<LittleEndian>,
    pub size_of_initialized_data: U32<LittleEndian>,
    pub size_of_uninitialized_data: U32<LittleEndian>,
    pub address_of_entry_point: U32<LittleEndian>,
    pub base_of_code: U32<LittleEndian>,
    pub base_of_data: U32<LittleEndian>,
    pub image_base: U32<LittleEndian>,
    pub section_alignment: U32<LittleEndian>,
    pub file_alignment: U32<LittleEndian>,
    pub major_operating_system_version: U16<LittleEndian>,
    pub minor_operating_system_version: U16<LittleEndian>,
    pub major_image_version: U16<LittleEndian>,
    pub minor_image_version: U16<LittleEndian>,
    pub major_subsystem_version: U16<LittleEndian>,
    pub minor_subsystem_version: U16<LittleEndian>,
    pub win32_version_value: U32<LittleEndian>,
    pub size_of_image: U32<LittleEndian>,
    pub size_of_headers: U32<LittleEndian>,
    pub checksum: U32<LittleEndian>,
    pub subsystem: U16<LittleEndian>,
    pub dll_characteristics: U16<LittleEndian>,
    pub size_of_stack_reserve: U32<LittleEndian>,
    pub size_of_stack_commit: U32<LittleEndian>,
    pub size_of_heap_reserve: U32<LittleEndian>,
    pub size_of_heap_commit: U32<LittleEndian>,
    pub loader_flags: U32<LittleEndian>,
    pub number_of_rva_and_sizes: U32<LittleEndian>,
    pub data_directory: [ImageDataDirectory; IMAGE_NUMBER_OF_DIRECTORY_ENTRIES]
}

#[repr(C)] 
#[derive(TryFromBytes, KnownLayout, Immutable, Debug, Clone, Copy)]
pub struct ImageFileHeader {
    pub machine: U16<LittleEndian>,
    pub number_of_sections: U16<LittleEndian>,
    pub time_date_stamp: U32<LittleEndian>,
    pub pointer_to_symbol_table: U32<LittleEndian>,
    pub number_of_symbols: U32<LittleEndian>,
    pub size_of_optional_header: U16<LittleEndian>,
    pub characteristics: U16<LittleEndian>, 
}

#[repr(C)]
#[derive(TryFromBytes, KnownLayout, Immutable, Debug, Clone, Copy)]
pub struct ImageNtHeadersx64 {
    pub signature: U32<LittleEndian>,
    pub file_header: ImageFileHeader,
    pub optional_header: ImageOptionalHeaderx64
}

#[repr(C)]
#[derive(TryFromBytes, KnownLayout, Immutable, Debug, Clone, Copy)]
pub struct ImageNtHeadersx86 {
    pub signature: U32<LittleEndian>,
    pub file_header: ImageFileHeader,
    pub optional_header: ImageOptionalHeaderx86
}

#[repr(C)]
#[derive(TryFromBytes, KnownLayout, Immutable, Debug, Clone, Copy)]
pub struct ImageSectionHeader {
    pub name: [u8; 8],
    pub misc: U32<LittleEndian>,
    pub virtual_address: U32<LittleEndian>,
    pub size_of_raw_data: U32<LittleEndian>,
    pub pointer_to_raw_data: U32<LittleEndian>,
    pub pointer_to_relocations: U32<LittleEndian>,
    pub pointer_to_linenumbers: U32<LittleEndian>,
    pub number_of_relocations: U16<LittleEndian>,
    pub number_of_linenumbers: U16<LittleEndian>,
    pub characteristics: U32<LittleEndian>,
}

#[repr(C)]
#[derive(TryFromBytes, KnownLayout, Immutable, Debug, Clone, Copy)]
pub struct ImageDosHeader {
    pub e_magic: U16<LittleEndian>,
    pub e_cblp: U16<LittleEndian>,
    pub e_cp: U16<LittleEndian>,
    pub e_crlc: U16<LittleEndian>,
    pub e_cparhdr: U16<LittleEndian>,
    pub e_minalloc: U16<LittleEndian>,
    pub e_maxalloc: U16<LittleEndian>,
    pub e_ss: U16<LittleEndian>,
    pub e_sp: U16<LittleEndian>,
    pub e_csum: U16<LittleEndian>,
    pub e_ip: U16<LittleEndian>,
    pub e_cs: U16<LittleEndian>,
    pub e_lfarlc: U16<LittleEndian>,
    pub e_ovno: U16<LittleEndian>,
    pub e_res: [U16<LittleEndian>; 4],
    pub e_oemid: U16<LittleEndian>,
    pub e_oeminfo: U16<LittleEndian>,
    pub e_res2: [U16<LittleEndian>; 10],
    pub e_lfanew: U32<LittleEndian>,
}

// get_arch helper struct
#[repr(C)]
#[derive(TryFromBytes, KnownLayout, Immutable, Debug, Clone, Copy)]
pub struct PeCommonHeader {
    pub signature: U32<LittleEndian>,
    pub machine: U16<LittleEndian>,
}