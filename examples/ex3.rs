
extern crate ebpf_toolchain;

use std::fs::File;
use std::io::Read;

use ebpf_toolchain::v1 as ebpf;

use ebpf::elf_loader::{
    EbpfProgram,
    ElfFile
};


fn main() {

    let file = "ebpf_prog_1.o";

    println!("Loading elf file: {}", file);
    let mut f = File::open(file).expect("Failed to open file!");
    let mut f_data = Vec::new();
    f.read_to_end(&mut f_data);
    let ef = ElfFile::new(f_data.as_slice()).expect("Failed to parse ELF file!");
    println!("ef = {:?}", ef);
    for section in ef.section_iter() {
        //println!("section = {:?}", section);
        println!("section name = {:?}", section.get_name(&ef));
    }
    let prog = EbpfProgram::from_elf_file(&ef);
}
