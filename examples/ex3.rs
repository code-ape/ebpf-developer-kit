
extern crate ebpf_development_kit;

use std::fs::File;
use std::io::Read;

use ebpf_development_kit::v1 as ebpf;

use ebpf::elf_loader as elf;

use elf::{
    EbpfProgramResource,
};

use ebpf::lowlevel::KernelInfo;

use ebpf::program as program;

use ebpf::socket_filter as socket_filter;

fn main() {

    let file = "ebpf_prog_1.o";

    // Read Elf file into variable f_data
    println!("Loading elf file: {}", file);
    let mut f = File::open(file).expect("Failed to open file!");
    let mut f_data = Vec::new();
    f.read_to_end(&mut f_data).unwrap();
    
    // Create ElfFile of data, this is needed to extract license and program bytes
    let ef = elf::File::new(f_data.as_slice()).expect("Failed to parse ELF file!");

    // Create elf::ProgramInfo, this holds all data needed to attempt loading
    // the program
    let efpi = elf::ProgramInfo {
        elf_file: ef,
        program_type: ebpf::lowlevel::ProgramType::SocketFilter,
        license_classifier: "license",
        program_classifier: "classifier"
    };
    let ef = efpi.attempt_load().expect("Failed to load program!");

    let kernel_info = KernelInfo::get().expect("Failed to get kernel info!");

    let li = program::LoadInfo {
        program: &ef,
        log_level: program::EbpfProgLoadLogLevel::Normal,
        kernel_release: kernel_info.release
    };

    println!("{:?}", li);

    let prog = li.attempt_load().expect("Failed to load program!");

    let raw_socket = socket_filter::open_raw_sock("lo").expect("Failed to open raw socket");

    println!("raw_socket = {:?}", raw_socket);

    socket_filter::attach_ebpf_filter(&raw_socket, prog).expect("Failed to attach filter");

    socket_filter::set_socket_rx_ring(&raw_socket, 1048576, 16, 1000)
        .expect("Failed set RX_RING");

    let packet_ring = socket_filter::mmap_rx_ring(&raw_socket, 1048576, 16, 1000)
        .expect("Failed set mmap rx ring");

    println!("packet_ring = {:?}", packet_ring)

}
