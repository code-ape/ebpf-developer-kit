
pub mod lowlevel;

use std::io;
use std::u32;

use ::v1::lowlevel::{
    KernelRelease,
};

use ::v1::program::lowlevel::load_program;

pub use ::v1::lowlevel::{
    ProgramType,
    EbpfProgLoadLogLevel,
    ProgramFd,
    ProgLoadAttr
};

use std::str;

#[derive(Debug)]
pub struct EbpfProgram<'a> {
    program_type: ProgramType,
    instructions: &'a [u64],
    license: &'a [u8],
}

impl<'a> EbpfProgram<'a> {
    pub fn new(program_type: ProgramType, instructions: &'a [u64], license: &'a [u8])
    -> EbpfProgram<'a> {
        EbpfProgram {
            program_type: program_type,
            instructions: instructions,
            license: license
        }
    }
}

type LoadResult = Result<ProgramFd, io::Error>;

#[derive(Debug)]
pub struct LoadInfo<'a> {
    pub program: &'a EbpfProgram<'a>,
    pub log_level: EbpfProgLoadLogLevel,
    pub kernel_release: KernelRelease
}

/*
#[repr(C)]
#[derive(Debug)]
pub struct ProgLoadAttr {
    prog_type: __u32,
    insn_cnt: __u32,
    insns: __aligned_u64,
    license: __aligned_u64,
    log_level: __u32,
    log_size: __u32,
    log_buf: __aligned_u64,
    kern_version: __u32
}
*/

impl<'a> LoadInfo<'a> {
    pub fn attempt_load(&self) -> LoadResult {

        //let debug_log : Vec<u8> = Vec::with_capacity(2<<20);
        let debug_log_array = [0 as u8; 8192];

        let prog_load_attr = ProgLoadAttr {
            prog_type: self.program.program_type.clone() as u32,
            insn_cnt: self.program.instructions.len() as u32,
            insns: &(self.program.instructions[0]) as *const u64 as u64,
            license: &(self.program.license[0]) as *const u8 as u64,
            log_level: self.log_level.clone() as u32,
            log_size: debug_log_array.len() as u32,
            log_buf: &(debug_log_array[0]) as *const u8 as u64,
            kern_version: 264489 //self.kernel_release.clone().into()
        };
        let r = unsafe { load_program(prog_load_attr) };

        let mut cs = str::from_utf8(&debug_log_array).expect("From utf8 failed");
        cs = cs.trim_right_matches('\0');
        println!("\n{}\n", cs);
        match r {
            Ok(prog_fd) => {
                Ok(prog_fd)
            },
            Err(error) => Err(error)
        }
    }
}
