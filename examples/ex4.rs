
extern crate ebpf_development_kit;
extern crate libc;

use std::fs::File;
use std::io::Read;
use std::mem;

use ebpf_development_kit::v1 as ebpf;


use elf::EbpfProgramResource;
use ebpf::lowlevel::KernelInfo;

use ebpf::{
    elf_loader as elf,
    program as program,
    socket_filter as socket_filter,
};


// 1) file -> bytes -> elf data -> program / license bytes (eBPF bytefiles)
// 2) eBPF bytes + prog. type + kernel info + debug level -> loaded eBPF program
// 3) socket options -> raw socket
// 4) loaded eBPF program + raw socket + interface name => running filter

fn main() {

    let file = "ebpf_prog_1.o";
    let interface = "wpl2s0";

    // Read Elf file into variable f_data
    println!("Loading elf file: {}", file);
    let mut f = File::open(file).expect("Failed to open file!");
    let mut f_data = Vec::new();
    f.read_to_end(&mut f_data).unwrap();
    
    // Create ElfFile of data, this is needed to extract license and program bytes
    let ef = elf::File::new(f_data.as_slice()).expect("Failed to parse ELF file!");

    // Create elf::ProgramInfo, this holds all data needed to attempt loading
    // the program
    let elf_pi = elf::ProgramInfo::<program::SocketFilter>::new(
        ef, "license", "classifier"
    );
    
    // Attempt loading eBPF program into kernel
    let ebpf_sf: program::SocketFilter = elf_pi.attempt_load()
        .expect("Failed to load program!");


    // setup socket
    // TODO: introduce idea of capture socket
    let socket_info = socket_filter::Info {
        interface: interface,
        packet_version: socket_filter::PacketVersion::V3,
        read_method: socket_filter::ReadMethod::RxRing{
            block_size: 32768,
            block_count: 4,
            ring_timeout: 1000
        },
        filter_program: ebpf_sf,
    };
    
    /*
    let filter = socket_filter::TryFromInfo(socket_info)
        .expect("Failed to set up ");

    
    for _ in 0..1000 {
        let (block, pr_tmp) = packet_ring.get_next();
        println!("\nblock = {:?}\n", block);
        for d in block.iter(&pr_tmp) {
            println!("{:?}", d);
            println!("{}", d);
        }
    }
    */
    
}
