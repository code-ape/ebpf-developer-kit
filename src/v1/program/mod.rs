
pub mod lowlevel;

use std::io;
use std::u32;
use std::marker::{
    PhantomData,
    Sized
};

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

type LoadResult<T> = Result<Box<T>, io::Error>;

#[derive(Debug)]
enum ProgramState {
    NotLoaded,
    Loaded,
    //Unloaded,
}

pub struct ProgramData<'a, T: EbpfProgram<'a>> {
    pub instructions: &'a [u64],
    pub license: &'a [u8],
    pub program_type: PhantomData<T>
}

// TODO: Implement into ProgramData for (instructions, license) tuple, unsure how to pass type?

pub trait EbpfProgram<'a> where Self: Sized {
    //fn new(instructions: &'a [u64], license: &'a [u8])-> Self;
    fn program_type() -> ProgramType;
    fn instructions(&self) -> &'a [u64];
    fn license(&self) -> &'a [u8];
    fn file_descriptor(&self) -> ProgramFd;
    fn attempt_kernel_load<T: Into<ProgramData<'a,Self>>>(
        T,
        log_level: EbpfProgLoadLogLevel
    ) -> LoadResult<Self>;
    fn attempt_kernel_load_for_kernel_release<T: Into<ProgramData<'a,Self>>>(
        T,
        log_level: EbpfProgLoadLogLevel,
        kernel_release: KernelRelease
    ) -> LoadResult<Self>;
}

#[derive(Debug)]
pub struct SocketFilter<'a> {
    instructions: &'a [u64],
    license: &'a [u8],
    //state: ProgramState,
    log_level: EbpfProgLoadLogLevel,
    kernel_release: KernelRelease,
    fd: ProgramFd,
    log_output: String,
}


impl<'a> EbpfProgram<'a> for SocketFilter<'a> {

    fn program_type() -> ProgramType {
        ProgramType::SocketFilter
    }
    fn license(&self) -> &'a [u8] {
        self.license
    }
    fn instructions(&self) -> &'a [u64] {
        self.instructions
    }
    fn file_descriptor(&self) -> ProgramFd {
        self.fd.clone()
    }
    
    fn attempt_kernel_load<T: Into<ProgramData<'a,Self>>>(
        prog_data_var: T, log_level: EbpfProgLoadLogLevel
    ) -> LoadResult<Self> {
        let kernel_info = KernelInfo::get().expect("Failed to get kernel info."); //TODO!!!!
        Self::attempt_kernel_load_for_kernel_release(
            prog_data_var, log_level, kernel_info.release
        )
    }

    fn attempt_kernel_load_for_kernel_release<T: Into<ProgramData<'a,Self>>>(
        prog_data_var: T,
        log_level: EbpfProgLoadLogLevel,
        kernel_release: KernelRelease
    ) -> LoadResult<Self> {

        let prog_data : ProgramData<Self> = prog_data_var.into();

        //let debug_log : Vec<u8> = Vec::with_capacity(2<<20);
        let debug_log_array = [0 as u8; 8192];

        let prog_load_attr = ProgLoadAttr {
            prog_type: Self::program_type() as u32,
            insn_cnt: prog_data.instructions.len() as u32,
            insns: &(prog_data.instructions[0]) as *const u64 as u64,
            license: &(prog_data.license[0]) as *const u8 as u64,
            log_level: log_level.clone() as u32,
            log_size: debug_log_array.len() as u32,
            log_buf: &(debug_log_array[0]) as *const u8 as u64,
            kern_version: kernel_release.clone().into()
        };
        println!("prog_load_attr = {:?}", prog_load_attr);
        let r = unsafe { load_program(prog_load_attr) };

        let mut cs = str::from_utf8(&debug_log_array).expect("From utf8 failed");
        cs = cs.trim_right_matches('\0');
        println!("\n{}\n", cs);
        match r {
            Ok(prog_fd) => {
                Ok(Box::new(SocketFilter{
                    instructions: prog_data.instructions,
                    license: prog_data.license,
                    log_level: log_level,
                    kernel_release: kernel_release,
                    fd: prog_fd,
                    log_output: String::from(cs),
                }))
            },
            Err(e) => {
                println!("Failed attempt_kernel_load, error: {}", e);
                Err(e)
            }
        }
    }
}
