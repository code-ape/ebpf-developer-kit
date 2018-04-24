
extern crate ebpf_development_kit;
extern crate libc;
extern crate env_logger;
extern crate log;

use std::fs::File;
use std::io::Read;
//# For custom env logger #//
//use std::io::Write;
//use std::env;
//use log::{Record, LevelFilter};
//use env_logger::{Builder, fmt};

use ebpf_development_kit::v1 as ebpf;

use ebpf::program::EbpfProgram;

use elf::EbpfProgramResource;
use ebpf::lowlevel::KernelInfo;

use ebpf::{
    elf_loader as elf,
    program as program,
    socket_filter as socket_filter,
};


fn main() {
    /*
    // Config logging
    let format = |buf: &mut fmt::Formatter, r: &Record| {
        let ln = match r.line() {
            Some(n) => n as i64,
            None => -1
        };
        writeln!(buf, "{}: {} (L{}): {}",
            r.level(),
            r.file().unwrap_or("?"),
            ln,
            r.args()
        )
    };

    let mut builder = Builder::new();
    builder.format(format).filter(None, LevelFilter::Info);

    if env::var("RUST_LOG").is_ok() {
       builder.parse(&env::var("RUST_LOG").unwrap());
    }

    builder.init();
    */
    env_logger::init();

    // TODO, put in libc issue
    //println!("size_of(SockAddrLL) = {}",
    //    mem::size_of::<socket_filter::SockAddrLL>());
    //println!("size_of(sockaddr) = {}",
    //    mem::size_of::<libc::sockaddr>());

    let file = "ebpf_prog_1.o";

    // Read Elf file into variable f_data
    //println!("Loading elf file: {}", file);
    let mut f = File::open(file).expect("Failed to open file!");
    let mut f_data = Vec::new();
    f.read_to_end(&mut f_data).unwrap();
    
    // Create ElfFile of data, this is needed to extract license and program bytes
    let ef = elf::File::new(f_data.as_slice()).expect("Failed to parse ELF file!");

    // Create elf::ProgramInfo, this holds all data needed to attempt loading
    // the program
    let prog_info = elf::ProgramInfo::<program::SocketFilter>::new(
        ef, "license", "classifier"
    );
    
    let prog_data: program::ProgramData<program::SocketFilter> = prog_info.attempt_load()
        .expect("Failed to load program!");

    //let kernel_info = KernelInfo::get().expect("Failed to get kernel info!");

    //let li = program::LoadInfo {
    //    program: &ef,
    //    log_level: program::EbpfProgLoadLogLevel::Normal,
    //    kernel_release: kernel_info.release
    //};

    //println!("{:?}", li);

    //let prog = program::SocketFilter::new();
    //li.attempt_load().expect("Failed to load program!");

    let prog = program::SocketFilter::attempt_kernel_load(
            prog_data, program::EbpfProgLoadLogLevel::Normal
        ).expect("Failed to load program into kernel.");

    let raw_socket = socket_filter::open_raw_sock().expect("Failed to open raw socket");
    
    socket_filter::set_packet_version_v3(&raw_socket)
        .expect("Failed to set packet version");
    
    socket_filter::set_socket_rx_ring(&raw_socket, 32768, 4, 1000)
        .expect("Failed set RX_RING");

    let mut packet_ring = socket_filter::mmap_rx_ring(&raw_socket, 32768, 4)
        .expect("Failed set mmap rx ring");
    

    socket_filter::attach_ebpf_filter(&raw_socket, prog)
        .expect("Failed to attach filter");

    socket_filter::bind_to_interface(&raw_socket, "wlp2s0")
        .expect("Failed to bind to interface");

    
    for _ in 0..1000 {
        let (block, pr_tmp) = packet_ring.get_next();
        println!("\nblock = {:?}\n", block);
        for d in block.iter(&pr_tmp) {
            println!("{:?}", d);
            println!("{}", d);
        }
    }

    
}
