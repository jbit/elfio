#![no_std]

#[macro_use]
mod macros;

use core::mem::size_of;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait ElfType: Clone + Copy + Default + Eq + PartialEq + DeserializeOwned + Serialize {}
impl ElfType for u32 {}
impl ElfType for u64 {}

/// ELF file header types
pub mod ehdr {
    use super::*;

    enum_struct!(
    /// ELF file class
    pub struct EIC(u8) {
        NONE    = 0 => "Invalid",
        ELF32   = 1 => "32-bit ELF",
        ELF64   = 2 => "64-bit ELF",
    }
    );
    enum_struct!(
    /// Data storage format
    pub struct EID(u8) {
        NONE    = 0 => "Unknown",
        LSB     = 1 => "Little-endian",
        MSB     = 2 => "Big-endian",
    }
    );
    enum_struct!(
    /// ELF specification version
    pub struct EIV(u8) {
        NONE    = 0 => "No version",
        CURRENT = 1 => "Current ELF version",
    }
    );
    enum_struct!(
    /// Operating system ABI
    pub struct EIOSABI(u8) {
        SYSV       = 0   => "System-V",
        HPUX       = 1   => "HP-UX",
        NETBSD     = 2   => "NetBSD",
        LINUX      = 3   => "Linux",
        SOLARIS    = 6   => "Sun Solaris",
        AIX        = 7   => "IBM AIX",
        IRIX       = 8   => "SGI Irix",
        FREEBSD    = 9   => "FreeBSD",
        TRU64      = 10  => "Compaq TRU64 UNIX",
        MODESTO    = 11  => "Novell Modesto",
        OPENBSD    = 12  => "OpenBSD",
        OPENVMS    = 13  => "DEC OpenVMS",
        NONSTOP    = 14  => "Tandem Nonstop Kernel",
        AROS       = 15  => "AROS Research Operating System",
        FENIX      = 16  => "Fenix",
        CLOUDABI   = 17  => "CloudABI",
        OPENVOS    = 18  => "Stratus OpenVOS",
        ARM_FDPIC  = 65  => "ARM FDPIC",
        ARM        = 97  => "ARM",
        STANDALONE = 255 => "Standalone",
    }
    );
    #[derive(Clone, Copy, Default, Eq, PartialEq, Deserialize, Serialize)]
    #[repr(C)]
    /// Object file file identification
    pub struct Eident {
        /// Always [`Eident::MAGIC`]
        pub magic: [u8; 4],
        /// 32bit or 64bit
        pub class: EIC,
        /// Little-endian or big-endian
        pub data: EID,
        /// Always [`EIV::CURRENT`]
        pub version: EIV,
        /// ABI the object is for
        pub osabi: EIOSABI,
        /// ABI version object is for
        pub abiversion: u8,
        /// Padding / future extension
        pub pad: [u8; 7],
    }
    impl Eident {
        /// The expected ELF file magic number (for [`Eident::magic`])
        pub const MAGIC: [u8; 4] = [0x7f, b'E', b'L', b'F'];
    }

    enum_struct!(
    /// Object file type
    pub struct ET(u16) {
        NONE   = 0      => "No type",
        REL    = 1      => "Relocatable object",
        EXEC   = 2      => "Executable object",
        DYN    = 3      => "Shared object",
        CORE   = 4      => "Core dump",
        LOOS   = 0xfe00 => "First operating system specific type",
        HIOS   = 0xfeff => "Last operating system specific type",
        LOPROC = 0xff00 => "First processor specific type",
        HIPROC = 0xffff => "Last processor specific type",
    }
    );

    enum_struct!(
    /// Object file machine architecture
    pub struct EM(u16) {
        NONE        = 0     => "No machine",
        SPARC       = 2     => "Sun SPARC",
        X86         = 3     => "Intel 80386",
        M68K        = 4     => "Motorola 68000",
        MIPS        = 8     => "MIPS big-endian",
        PPC32       = 20    => "PowerPC 32-bit",
        PPC64       = 21    => "PowerPC 64-bit",
        ARM         = 40    => "ARM 32-bit",
        SPARCV9     = 43    => "Sun SPARC v9 (64-bit)",
        X86_64      = 62    => "AMD x86-64",
        AVR         = 83    => "Atmel AVR",
        OPENRISC    = 92    => "OpenRISC",
        XTENSA      = 94    => "Tensilica Xtensa",
        HEXAGON     = 164   => "Qualcomm Hexagon DSP",
        AARCH64     = 183   => "ARM 64-bit",
        RISCV       = 243   => "RISC-V",
        BPF         = 247   => "Linux BPF",
    }
    );

    enum_struct!(
    /// Object file version
    pub struct EV(u32) {
        NONE    = 0 => "No version",
        CURRENT = 1 => "Current version",
    }
    );

    flag_struct!(
    /// Target specific flags
    pub struct EF(u32) {
        NONE = 0 => "No flags",
    }
    );
}

/// ELF file header
#[derive(Clone, Copy, Default, Eq, PartialEq, Deserialize, Serialize)]
#[repr(C)]
pub struct Ehdr<T: ElfType> {
    /// Object file file identification
    pub e_ident: ehdr::Eident,
    /// Object file type
    pub e_type: ehdr::ET,
    /// Object file machine architecture
    pub e_machine: ehdr::EM,
    /// Object file version
    pub e_version: ehdr::EV,
    /// Entry point address
    #[serde(bound = "")]
    pub e_entry: T,
    /// Offset of program header table
    #[serde(bound = "")]
    pub e_phoff: T,
    /// Offset of Section header table
    #[serde(bound = "")]
    pub e_shoff: T,
    /// Target specific flags
    pub e_flags: ehdr::EF,
    /// Size of ELF header
    pub e_ehsize: u16,
    /// Size of program header table entry
    pub e_phentsize: u16,
    /// Number of program headers
    pub e_phnum: u16,
    /// Size of section header table entry
    pub e_shentsize: u16,
    /// Number of section headers
    pub e_shnum: u16,
    /// Index of string table in section header table
    pub e_shstrndx: u16,
}
impl<T: ElfType> Ehdr<T> {
    pub const SIZE: usize = size_of::<Self>();
}
impl From<Ehdr32> for Ehdr64 {
    fn from(o: Ehdr32) -> Ehdr64 {
        Ehdr64 {
            e_ident: o.e_ident,
            e_type: o.e_type,
            e_machine: o.e_machine,
            e_version: o.e_version,
            e_entry: o.e_entry as u64,
            e_phoff: o.e_phoff as u64,
            e_shoff: o.e_shoff as u64,
            e_flags: o.e_flags,
            e_ehsize: o.e_ehsize,
            e_phentsize: o.e_phentsize,
            e_phnum: o.e_phnum,
            e_shentsize: o.e_shentsize,
            e_shnum: o.e_shnum,
            e_shstrndx: o.e_shstrndx,
        }
    }
}

pub type Ehdr32 = Ehdr<u32>;
pub type Ehdr64 = Ehdr<u64>;

/// Program header types
pub mod phdr {
    enum_struct!(
    /// Segment type
    pub struct PT(u32) {
        NULL    = 0          => "Unused",
        LOAD    = 1          => "Loadable",
        DYNAMIC = 2          => "Dynamic linking information",
        INTERP  = 3          => "Program interpreter",
        NOTE    = 4          => "Auxiliary information",
        SHLIB   = 5          => "Reserved",
        PHDR    = 6          => "Program header table",
        LOOS    = 0x60000000 => "First operating system specific type",
        HIOS    = 0x6fffffff => "Last operating system specific type",
        LOPROC  = 0x70000000 => "First processor specific type",
        HIPROC  = 0x7fffffff => "Last processor specific type",
    }
    );

    flag_struct!(
    /// Segment flags
    pub struct PF(u32) {
        NONE = 0 => "No flags",
        X    = 1 => "Executable",
        W    = 2 => "Writable",
        R    = 4 => "Readable",
    }
    );
}

// Trait for Phdr32 and Phdr64
pub trait Phdr: Clone + Copy + Default + Eq + PartialEq + DeserializeOwned + Serialize {
    const SIZE: usize = size_of::<Self>();
    type ElfType: ElfType;
}

/// 32-bit program header
#[derive(Clone, Copy, Default, Eq, PartialEq, Deserialize, Serialize)]
#[repr(C)]
pub struct Phdr32 {
    /// Segment type
    pub p_type: phdr::PT,
    /// File offset of segment data
    pub p_offset: u32,
    /// Virtual memory address of segment
    pub p_vaddr: u32,
    /// Physical memory address of segment
    pub p_paddr: u32,
    /// Size of segment data in file
    pub p_filesz: u32,
    /// Size of segment in memory
    pub p_memsz: u32,
    /// Segment flags
    pub p_flags: phdr::PF,
    /// Segment alignment
    pub p_align: u32,
}
impl Phdr for Phdr32 {
    type ElfType = u32;
}

/// 64-bit program header
#[derive(Clone, Copy, Default, Eq, PartialEq, Deserialize, Serialize)]
#[repr(C)]
pub struct Phdr64 {
    /// Segment type
    pub p_type: phdr::PT,
    /// Segment flags
    pub p_flags: phdr::PF,
    /// File offset of segment data
    pub p_offset: u64,
    /// Virtual memory address of segment
    pub p_vaddr: u64,
    /// Physical memory address of segment
    pub p_paddr: u64,
    /// Size of segment data in file
    pub p_filesz: u64,
    /// Size of segment in memory
    pub p_memsz: u64,
    /// Segment alignment
    pub p_align: u64,
}
impl Phdr for Phdr64 {
    type ElfType = u64;
}
impl From<Phdr32> for Phdr64 {
    fn from(o: Phdr32) -> Phdr64 {
        Phdr64 {
            p_type: o.p_type,
            p_flags: o.p_flags,
            p_offset: o.p_offset as u64,
            p_vaddr: o.p_vaddr as u64,
            p_paddr: o.p_paddr as u64,
            p_filesz: o.p_filesz as u64,
            p_memsz: o.p_memsz as u64,
            p_align: o.p_align as u64,
        }
    }
}

/// Section header types
pub mod shdr {
    enum_struct!(
    /// Section type
    pub struct SHT(u32) {
        NULL          = 0          => "Unused",
        PROGBITS      = 1          => "Program data",
        SYMTAB        = 2          => "Symbol table",
        STRTAB        = 3          => "String table",
        RELA          = 4          => "Relocation entries, with addends",
        HASH          = 5          => "Symbol hash table",
        DYNAMIC       = 6          => "Dynamic linking information",
        NOTE          = 7          => "Notes",
        NOBITS        = 8          => "Program space with no data (BSS)",
        REL           = 9          => "Relocation entries, no addends",
        SHLIB         = 10         => "Reserved",
        DYNSYM        = 11         => "Dynamic linker symbol table",
        INIT_ARRAY    = 14         => "Constructors",
        FINI_ARRAY    = 15         => "Destructors",
        PREINIT_ARRAY = 16         => "Pre-constructors",
        GROUP         = 17         => "Section group",
        SYMTAB_SHNDX  = 18         => "Extended",
        LOOS          = 0x60000000 => "First operating system specific type",
        HIOS          = 0x6fffffff => "Last operating system specific type",
        LOPROC        = 0x70000000 => "First processor specific type",
        HIPROC        = 0x7fffffff => "Last processor specific type",
        LOUSER        = 0x80000000 => "First user specific type",
        HIUSER        = 0x8fffffff => "Last user specific type",
    }
    );

    flag_struct!(
    /// Section flags (32-bit)
    pub struct SHF32(u32) {
        NONE = 0 => "No flags",
    }
    );

    flag_struct!(
    /// Section flags (64-bit)
    pub struct SHF64(u64) {
        NONE = 0 => "No flags",
    }
    );
    impl From<SHF32> for SHF64 {
        fn from(other: SHF32) -> SHF64 {
            SHF64(other.0 as u64)
        }
    }
}

// Trait for Shdr32 and Shdr64
pub trait Shdr: Clone + Copy + Default + Eq + PartialEq + DeserializeOwned + Serialize {
    const SIZE: usize = size_of::<Self>();
    type ElfType: ElfType;
}

/// 32-bit section header
#[derive(Clone, Copy, Default, Eq, PartialEq, Deserialize, Serialize)]
#[repr(C)]
pub struct Shdr32 {
    /// Section name (string table index)
    pub sh_name: u32,
    /// Section type
    pub sh_type: shdr::SHT,
    /// Section flags
    pub sh_flags: shdr::SHF32,
    /// Virtual memory address of section data
    pub sh_addr: u32,
    /// File offset of section data
    pub sh_offset: u32,
    /// Size of section data
    pub sh_size: u32,
    /// Index of another section
    pub sh_link: u32,
    /// Additional section information
    pub sh_info: u32,
    /// Section memory alignment
    pub sh_addralign: u32,
    /// Size of internal table entries
    pub sh_entsize: u32,
}
impl Shdr for Shdr32 {
    type ElfType = u32;
}

/// 64-bit section header
#[derive(Clone, Copy, Default, Eq, PartialEq, Deserialize, Serialize)]
#[repr(C)]
pub struct Shdr64 {
    /// Section name (string table index)
    pub sh_name: u32,
    /// Section type
    pub sh_type: shdr::SHT,
    /// Section flags
    pub sh_flags: shdr::SHF64,
    /// Virtual memory address of section data
    pub sh_addr: u64,
    /// File offset of section data
    pub sh_offset: u64,
    /// Size of section data
    pub sh_size: u64,
    /// Index of another section
    pub sh_link: u32,
    /// Additional section information
    pub sh_info: u32,
    /// Section memory alignment
    pub sh_addralign: u64,
    /// Size of internal table entries
    pub sh_entsize: u64,
}
impl Shdr for Shdr64 {
    type ElfType = u64;
}
impl From<Shdr32> for Shdr64 {
    fn from(o: Shdr32) -> Shdr64 {
        Shdr64 {
            sh_name: o.sh_name,
            sh_type: o.sh_type,
            sh_flags: o.sh_flags.into(),
            sh_addr: o.sh_addr as u64,
            sh_offset: o.sh_offset as u64,
            sh_size: o.sh_size as u64,
            sh_link: o.sh_link,
            sh_info: o.sh_info,
            sh_addralign: o.sh_addralign as u64,
            sh_entsize: o.sh_entsize as u64,
        }
    }
}

/// Common section names
pub mod section {
    /// Zeroed data
    pub const BSS: &str = ".bss";
    /// Data from file
    pub const DATA: &str = ".data";
    /// Debug information
    pub const DEBUG: &str = ".debug";
    /// Dynamic linking information
    pub const DYNAMIC: &str = ".dynamic";
    /// Dynamic linking strings
    pub const DYNSTR: &str = ".dynstr";
    /// Dynamic linking symbols
    pub const DYNSYM: &str = ".dynsym";
    /// Destructors
    pub const FINI: &str = ".fini";
    /// Global offsets
    pub const GOT: &str = ".got";
    /// Symbol hashes
    pub const HASH: &str = ".hash";
    /// Constructors
    pub const INIT: &str = ".init";
    /// Relocation data
    pub const REL_DATA: &str = ".rel.data";
    /// Relocation destructors
    pub const REL_FINI: &str = ".rel.fini";
    /// Relocation consructors
    pub const REL_INIT: &str = ".rel.init";
    /// Relocation dynamic linking information
    pub const REL_DYN: &str = ".rel.dyn";
    /// Read-only relocation data
    pub const REL_RODATA: &str = ".rel.rodata;";
    /// Relocation code
    pub const REL_TEXT: &str = ".rel.text";
    /// Read-only data
    pub const RODATA: &str = ".rodata";
    /// Section header strings
    pub const SHSTRTAB: &str = ".shstrtab";
    /// Strings
    pub const STRTAB: &str = ".strtab";
    /// Symbols
    pub const SYMTAB: &str = ".symtab";
    /// Executable code
    pub const TEXT: &str = ".text";
}

#[cfg(test)]
mod test {
    use super::*;

    fn serialized_size(t: &impl Serialize) -> usize {
        bincode::serialized_size(t).unwrap() as usize
    }

    #[test]
    fn ehdr_size() {
        assert_eq!(Ehdr32::SIZE, 0x34);
        assert_eq!(Ehdr32::SIZE, serialized_size(&Ehdr32::default()));
        assert_eq!(Ehdr64::SIZE, 0x40);
        assert_eq!(Ehdr64::SIZE, serialized_size(&Ehdr64::default()));
    }

    #[test]
    fn phdr_size() {
        assert_eq!(Phdr32::SIZE, 0x20);
        assert_eq!(Phdr32::SIZE, serialized_size(&Phdr32::default()));
        assert_eq!(Phdr64::SIZE, 0x38);
        assert_eq!(Phdr64::SIZE, serialized_size(&Phdr64::default()));
    }
    #[test]
    fn hdr_size() {
        assert_eq!(Shdr32::SIZE, 0x28);
        assert_eq!(Shdr32::SIZE, serialized_size(&Shdr32::default()));
        assert_eq!(Shdr64::SIZE, 0x40);
        assert_eq!(Shdr64::SIZE, serialized_size(&Shdr64::default()));
    }
}
