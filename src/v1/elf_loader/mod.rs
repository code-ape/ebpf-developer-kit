use xmas_elf as elf;

use std;
//use std::fs;
//use std::io;
//use std::io::Read;
use std::marker::PhantomData;

use self::elf::sections::{
    SectionData
};

//use ::v1::lowlevel::ProgramType;

use ::v1::program as program;
use self::program::{
    EbpfProgram,
    ProgramData
};

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

// E0122
//type LoadResult<'a, T: EbpfProgram<'a>> = Result<T, LoadError<'a>>;

pub trait EbpfProgramResource<'a, T: EbpfProgram<'a>> {
    fn attempt_load(&self) -> Result<ProgramData<'a,T>, LoadError<'a>>;
}

pub struct ProgramInfo<'a, T: EbpfProgram<'a>> {
    elf_file: File<'a>,
    license_classifier: &'a str,
    program_classifier: &'a str,
    // phantom program type
    program_type: PhantomData<T>,
}

impl<'a, T: EbpfProgram<'a>> ProgramInfo<'a, T> {
    pub fn new(
        elf_file: File<'a>,
        license_classifier: &'a str,
        program_classifier: &'a str
    ) -> Self {
        ProgramInfo {
            elf_file: elf_file,
            license_classifier: license_classifier,
            program_classifier: program_classifier,
            program_type: PhantomData
        }
    }
}

impl<'a, T: EbpfProgram<'a>> EbpfProgramResource<'a, T> for ProgramInfo<'a, T> {
    fn attempt_load(&self) -> Result<ProgramData<'a,T>, LoadError<'a>> {

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

        Ok(ProgramData { instructions: p_bytes, license: l_bytes, program_type: PhantomData })
    }
}

fn extract_undefined_section(section: SectionData)
    -> Result<&[u8],LoadError> {
    match section {
        SectionData::Undefined(data) => Ok(data),
        _ => Err(LoadError::CantGetSectionBytes)
    }
}

