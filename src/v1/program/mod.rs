
pub mod lowlevel;

use std::io;
use std::u32;

use ::v1::lowlevel::{
    KernelInfo,
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

type LoadResult = Result<(), io::Error>;

#[derive(Debug)]
enum ProgramState {
    NotLoaded,
    Loaded,
    Unloaded,
}

pub trait EbpfProgram<'a> {
    fn new(instructions: &'a [u64], license: &'a [u8])-> Self;
    fn program_type() -> ProgramType;
    fn instructions(&self) -> &'a [u64];
    fn license(&self) -> &'a [u8];
    fn attempt_load(
        &mut self,
        log_level: EbpfProgLoadLogLevel
    ) -> LoadResult;
    fn attempt_load_for_kernel_release(
        &mut self,
        log_level: EbpfProgLoadLogLevel,
        kernel_release: KernelRelease
    ) -> LoadResult;
}

#[derive(Debug)]
pub struct SocketFilter<'a> {
    instructions: &'a [u64],
    license: &'a [u8],
    state: ProgramState,
    log_level: Option<EbpfProgLoadLogLevel>,
    kernel_release: Option<KernelRelease>,
    fd: Option<ProgramFd>,
    log_output: Option<String>,
}


impl<'a> EbpfProgram<'a> for SocketFilter<'a> {
    fn new(instructions: &'a [u64], license: &'a [u8])
    -> SocketFilter<'a> {
        SocketFilter {
            instructions: instructions,
            license: license,
            state: ProgramState::NotLoaded,
            log_level: None,
            kernel_release: None,
            fd: None,
            log_output: None
        }
    }

    fn program_type() -> ProgramType {
        ProgramType::SocketFilter
    }
    fn license(&self) -> &'a [u8] {
        self.license
    }
    fn instructions(&self) -> &'a [u64] {
        self.instructions
    }

    fn attempt_load(&mut self, log_level: EbpfProgLoadLogLevel) -> LoadResult {
        let kernel_info = KernelInfo::get().unwrap(); //TODO!!!!
        self.attempt_load_for_kernel_release(
            log_level, kernel_info.release
        )
    }

    fn attempt_load_for_kernel_release(
        &mut self,
        log_level: EbpfProgLoadLogLevel,
        kernel_release: KernelRelease
    ) -> LoadResult {

        //let debug_log : Vec<u8> = Vec::with_capacity(2<<20);
        let debug_log_array = [0 as u8; 8192];

        let prog_load_attr = ProgLoadAttr {
            prog_type: Self::program_type() as u32,
            insn_cnt: self.instructions().len() as u32,
            insns: &(self.instructions()[0]) as *const u64 as u64,
            license: &(self.license()[0]) as *const u8 as u64,
            log_level: log_level.clone() as u32,
            log_size: debug_log_array.len() as u32,
            log_buf: &(debug_log_array[0]) as *const u8 as u64,
            kern_version: kernel_release.clone().into()
        };

        let r = unsafe { load_program(prog_load_attr) };

        let mut cs = str::from_utf8(&debug_log_array).expect("From utf8 failed");
        cs = cs.trim_right_matches('\0');
        println!("\n{}\n", cs);
        match r {
            Ok(prog_fd) => {
                self.fd = Some(prog_fd);
                self.log_output = Some(String::from(cs));
                self.state = ProgramState::Loaded;
                self.log_level = Some(log_level);
                Ok(())
            },
            Err(error) => Err(error)
        }
    }
}
