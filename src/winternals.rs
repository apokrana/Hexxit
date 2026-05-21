use zerocopy::{TryFromBytes, KnownLayout, Immutable, LittleEndian, U16, U32};
#[repr(C)] 
#[derive(TryFromBytes, KnownLayout, Immutable, Debug, Clone, Copy)]
pub struct ImageDataDirectory {
    pub virtual_address: u32,
    pub size: u32,
}

const IMAGE_NUMBER_OF_DIRECTORY_ENTRIES: usize = 16;
#[repr(C)] 
#[derive(TryFromBytes, KnownLayout, Immutable, Debug, Clone, Copy)]
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


#[repr(C)] 
#[derive(TryFromBytes, KnownLayout, Immutable, Debug, Clone, Copy)]
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

#[repr(C)] 
#[derive(TryFromBytes, KnownLayout, Immutable, Debug, Clone, Copy)]
pub struct ImageFileHeader {
    pub machine: u16,
    pub number_of_sections: u16,
    pub time_date_stamp: u32,
    pub pointer_to_symbol_table: u32,
    pub number_of_symbols: u32,
    pub size_of_optional_header: u16,
    pub characteristics: u16, 
}

#[repr(C)]
#[derive(TryFromBytes, KnownLayout, Immutable, Debug, Clone, Copy)]
pub struct ImageSectionHeader {
    pub name: [u8; 8],
    pub misc: u32,
    pub virtual_address: u32,
    pub size_of_raw_data: u32,
    pub pointer_to_raw_data: u32,
    pub pointer_to_relocations: u32,
    pub pointer_to_linenumbers: u32,
    pub number_of_relocations: u16,
    pub number_of_linenumbers: u16,
    pub characteristics: u32,
}

#[repr(C)]
#[derive(TryFromBytes, KnownLayout, Immutable, Debug, Clone, Copy)]
pub struct ImageNtHeadersx64 {
    pub signature: u32,
    pub file_header: ImageFileHeader,
    pub optional_header: ImageOptionalHeaderx64
}

#[repr(C)]
#[derive(TryFromBytes, KnownLayout, Immutable, Debug, Clone, Copy)]
pub struct ImageNtHeadersx86 {
    pub signature: u32,
    pub file_header: ImageFileHeader,
    pub optional_header: ImageOptionalHeaderx86
}

pub const IMAGE_DOS_SIGNATURE: u16 = 0x5A4D; // MZ

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