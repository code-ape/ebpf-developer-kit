 use xmas_elf as elf;

 use ::v1::lowlevel::{
    ProgramType,
 };

 pub use xmas_elf::ElfFile;

 pub struct EbpfProgram<'a> {
    program_type: ProgramType,
    instructions: &'a [usize],
    license: [char;8],
    log_level: u32,
    kern_version: (u8,u8,u8) //TODO: make this a struct
 }

impl<'a> EbpfProgram<'a> {
    pub fn from_elf_file(elf_file: &elf::ElfFile) -> Self {
        EbpfProgram {
            program_type: ProgramType::Unspec, //TODO
            instructions: &[],
            license: ['G','P','L',' ',' ',' ',' ',' '],
            log_level: 1,
            kern_version: (0,0,0)
        }
    }
}
