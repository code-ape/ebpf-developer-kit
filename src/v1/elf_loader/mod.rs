 use xmas_elf as elf;

 use ::v1::lowlevel::{
    ProgramType,
    KernelRelease,
    KernelInfo
 };

 pub use xmas_elf::ElfFile;

 pub struct EbpfProgram<'a> {
    program_type: ProgramType,
    instructions: &'a [usize],
    license: [char;8],
    log_level: u32,
    kern_version: KernelRelease
    
 }

impl<'a> EbpfProgram<'a> {
    pub fn from_elf_file(elf_file: &elf::ElfFile) -> Option<Self> {
        Some(EbpfProgram {
            program_type: ProgramType::Unspec, //TODO
            instructions: &[],
            license: ['G','P','L',' ',' ',' ',' ',' '],
            log_level: 1,
            kern_version: match KernelInfo::get() {
                Some(ki) => ki.release,
                None => return None
            }
        })
    }
}
