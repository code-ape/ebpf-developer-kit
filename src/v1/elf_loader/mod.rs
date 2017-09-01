use xmas_elf as elf;

use std;

use self::elf::sections::{
    SectionData
};

use ::v1::lowlevel::{
    ProgramType,
    KernelInfo,
    EbpfProgLoadLogLevel,
};

use ::v1::program::EbpfProgram;

pub use xmas_elf::ElfFile as File;

#[derive(Debug)]
pub enum LoadError<'a> {
    Unspec,
    Other(&'a str),
    CantGetSection(&'a str),
    CantGetSectionData(&'a str),
    CantGetSectionBytes,
    PartialInstruction
}

type LoadResult<'a> = Result<EbpfProgram<'a>, LoadError<'a>>;

pub trait EbpfProgramResource<'a> {
    fn attempt_load(&self) -> LoadResult<'a>;
}

pub struct ProgramInfo<'a> {
    pub elf_file: File<'a>,
    pub program_type: ProgramType,
    pub license_classifier: &'a str,
    pub program_classifier: &'a str
}

impl<'a> EbpfProgramResource<'a> for ProgramInfo<'a> {
    fn attempt_load(&self) -> LoadResult<'a> {

        let l_section = 
            self.elf_file
                .find_section_by_name(&self.license_classifier)
                .ok_or(LoadError::CantGetSection(&self.license_classifier))?;

        let l_section_data = match l_section.get_data(&self.elf_file) {
            Ok(data) => Ok(data),
            Err(error) => Err(LoadError::CantGetSectionData(error))
        }?;

        let l_bytes = extract_undefined_section(l_section_data)?;

        let p_section = 
            self.elf_file
                .find_section_by_name(&self.program_classifier)
                .ok_or(LoadError::CantGetSection(&self.program_classifier))?;

        let p_section_data = match p_section.get_data(&self.elf_file) {
            Ok(data) => Ok(data),
            Err(error) => Err(LoadError::CantGetSectionData(error))
        }?;

        let p_bytes_u8 = extract_undefined_section(p_section_data)?;


        let p_bytes : &[u64] = match p_bytes_u8.len() % 8 {
            0 => unsafe {
                Ok(std::slice::from_raw_parts(
                    &(p_bytes_u8[0]) as *const u8 as *const u64,
                    p_bytes_u8.len() / 8
                ))
            },
            _ => Err(LoadError::PartialInstruction)
        }?;

        println!("license_bytes = {:?}", l_bytes);
        println!("program_bytes = {:?}", p_bytes);

        Ok(EbpfProgram::new(
            self.program_type.clone(),
            p_bytes,
            l_bytes
        ))
    }
}

fn extract_undefined_section(section: SectionData)
    -> Result<&[u8],LoadError> {
    match section {
        SectionData::Undefined(data) => Ok(data),
        _ => Err(LoadError::CantGetSectionBytes)
    }
}


