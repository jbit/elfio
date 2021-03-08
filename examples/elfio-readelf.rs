use elfio::ehdr::*;
use elfio::*;
use serde::de::DeserializeOwned;
use std::env::args_os;
use std::io::{BufReader, Read};
use std::io::{Seek, SeekFrom};
use std::path::Path;
use std::process::exit;
use std::{ffi::OsStr, fs::File};

fn deserialize<T: DeserializeOwned>(
    reader: &mut dyn Read,
    big_endian: bool,
) -> Result<T, std::io::Error> {
    use bincode::Options;

    let options = bincode::DefaultOptions::new()
        .with_fixint_encoding()
        .allow_trailing_bytes();

    let result: Result<T, _> = if big_endian {
        options.with_big_endian().deserialize_from(reader)
    } else {
        options.with_little_endian().deserialize_from(reader)
    };

    match result {
        Err(err) => {
            if let bincode::ErrorKind::Io(ioerror) = *err {
                Err(ioerror)
            } else {
                Err(std::io::Error::new(std::io::ErrorKind::Other, err))
            }
        }
        Ok(t) => Ok(t),
    }
}

fn main() {
    let args: Vec<_> = args_os().collect();
    if args.len() != 2 {
        let name = args
            .get(0)
            .and_then(|s| Path::new(s).file_name())
            .unwrap_or(OsStr::new(env!("CARGO_CRATE_NAME")));
        println!("Usage:\n\t{} elf-file", name.to_string_lossy());
        exit(1);
    }
    let filename = Path::new(&args[1]);
    let f = File::open(filename).unwrap();
    let mut f = BufReader::new(f);

    let ident: Eident = deserialize(&mut f, false).unwrap();
    assert_eq!(ident.magic, Eident::MAGIC);
    assert!(ident.class == EIC::ELF32 || ident.class == EIC::ELF64);
    assert!(ident.data == EID::LSB || ident.data == EID::MSB);
    assert_eq!(ident.version, EIV::CURRENT);

    let elf32 = ident.class == EIC::ELF32;
    let big_endian = ident.data == EID::MSB;

    f.seek(SeekFrom::Start(0)).unwrap();

    let ehdr = if elf32 {
        // Convert ELF32 header into ELF64 header
        deserialize::<Ehdr32>(&mut f, big_endian).unwrap().into()
    } else {
        deserialize::<Ehdr64>(&mut f, big_endian).unwrap()
    };

    println!("ELF Header");
    println!("----------");
    println!("  Class:       {}", ehdr.e_ident.class,);
    println!("  Data:        {}", ehdr.e_ident.data,);
    println!("  Version:     {}", ehdr.e_ident.version);
    println!("  OS/ABI:      {}", ehdr.e_ident.osabi);
    println!("  ABI Version: {}", ehdr.e_ident.abiversion);
    println!("  Type:        {}", ehdr.e_type,);
    println!("  Machine:     {}", ehdr.e_machine);
    println!("  Version:     {}", ehdr.e_version);
    println!("  Entry point: 0x{:x}", ehdr.e_entry);
    println!("  Phdr offset: {}", ehdr.e_phoff);
    println!("  Shdr offset: {}", ehdr.e_shoff);
    println!("  Flags:       {:?}", ehdr.e_flags);
    println!("  Ehdr size:   {}", ehdr.e_ehsize);
    println!("  Phdr size:   {}", ehdr.e_phentsize);
    println!("  Phdr count:  {}", ehdr.e_phnum);
    println!("  Shdr size:   {}", ehdr.e_shentsize);
    println!("  Shdr count:  {}", ehdr.e_shnum);
    println!("  Shdr strtab: Section {}", ehdr.e_shstrndx);
    println!();
    if elf32 {
        assert_eq!(ehdr.e_ehsize as usize, Ehdr32::SIZE);
        assert!(ehdr.e_phnum == 0 || ehdr.e_phentsize as usize == Phdr32::SIZE);
        assert!(ehdr.e_shnum == 0 || ehdr.e_shentsize as usize == Shdr32::SIZE);
    } else {
        assert_eq!(ehdr.e_ehsize as usize, Ehdr64::SIZE);
        assert!(ehdr.e_phnum == 0 || ehdr.e_phentsize as usize == Phdr64::SIZE);
        assert!(ehdr.e_shnum == 0 || ehdr.e_shentsize as usize == Shdr64::SIZE);
    }

    println!("Program Headers");
    println!("---------------");

    println!("");
}
