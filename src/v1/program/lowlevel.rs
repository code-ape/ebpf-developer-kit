
use std::io::Error;

use ::v1::lowlevel::{
    ProgLoadAttr,
    //MapElemAttr,
    ProgramFd,
    ebpf_syscall,
    Action
};

pub unsafe fn load_program(prog_load_attr: ProgLoadAttr)
    -> Result<ProgramFd,Error> {
    match ebpf_syscall(Action::ProgLoad, prog_load_attr) {
        n if n > 0 => Ok(ProgramFd::new(n as i32)),
        -1 => Err(Error::last_os_error()),
        n => unreachable!("Syscall returned number other than 0 or 1: {}", n)
    }
}