use xmas_elf as elf;

use self::elf::sections::{
    SectionHeader,
    SectionData
};

use ::v1::lowlevel::{
    ProgramType,
    KernelRelease,
    KernelInfo,
};

pub use ::v1::lowlevel::EbpfProgLoadLogLevel;


pub use xmas_elf::ElfFile;

#[derive(Debug)]
pub struct EbpfProgram<'a> {
    program_type: ProgramType,
    instructions: &'a [usize],
    license: [char;8],
    log_level: EbpfProgLoadLogLevel,
    kern_version: KernelRelease
}

#[derive(Debug)]
pub enum EbpfProgramError<'a> {
    Unspec,
    CantGetSection(&'a str),
    CantGetSectionData(&'a str),
    CantGetSectionBytes,
    CantGetKernelRelease,
}

type EbpfProgResult<'a> = Result<EbpfProgram<'a>, EbpfProgramError<'a>>;

fn get_elf_section<'a,'b>(elf_file: &'b ElfFile, section_name: &'a str)
    -> Result<SectionHeader<'b>,EbpfProgramError<'a>> {
    Ok(
        elf_file.find_section_by_name(section_name).ok_or(
            EbpfProgramError::CantGetSection(section_name))?
    )
}

fn get_elf_section_data<'a,'b>(elf_file: &'b ElfFile, section: &'b SectionHeader)
    -> Result<SectionData<'b>,EbpfProgramError<'a>> {
    match section.get_data(elf_file) {
        Ok(data) => Ok(data),
        Err(error) => Err(EbpfProgramError::CantGetSectionData(error))
    }
}

fn extract_undefined_section<'a,'b>(section: &'b SectionData) -> Result<&'b [u8],EbpfProgramError<'a>> {
    match *section {
        SectionData::Undefined(data) => Ok(data),
        _ => Err(EbpfProgramError::CantGetSectionBytes)
    }
}

impl<'a> EbpfProgram<'a> {
    pub fn from_elf_file(elf_file: &ElfFile,
                         license_classifier: &'a str,
                         program_classifier: &'a str) -> EbpfProgResult<'a> {
        let license_section = get_elf_section(elf_file, license_classifier)?;
        let license_data = get_elf_section_data(elf_file, &license_section)?;
        let license_bytes = extract_undefined_section(&license_data)?;

        let program_section = get_elf_section(elf_file, program_classifier)?;

        let program_data = get_elf_section_data(elf_file, &program_section)?;

        println!("license_data = {:?}", license_data);



        Ok(EbpfProgram {
            program_type: ProgramType::Unspec, //TODO
            instructions: &[],
            license: ['G','P','L',' ',' ',' ',' ',' '],
            log_level: EbpfProgLoadLogLevel::Normal,
            kern_version: KernelInfo::get()
                            .ok_or(EbpfProgramError::CantGetKernelRelease)?
                            .release,
        })
    }
}
