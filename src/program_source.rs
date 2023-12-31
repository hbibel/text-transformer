use std::{fmt::Display, fs};

#[derive(Debug, PartialEq, Eq)]
pub enum ProgramSource {
    Literal(String),
    File(String),
}

impl Display for ProgramSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgramSource::File(fp) => f.write_fmt(format_args!("file {}", fp)),
            ProgramSource::Literal(l) => f.write_fmt(format_args!("literal string '{}'", l)),
        }
    }
}

pub fn code(source: ProgramSource) -> Result<String, String> {
    match source {
        ProgramSource::Literal(s) => Ok(s),
        ProgramSource::File(fp) => fs::read_to_string(fp.clone())
            .map_err(|e| format!("Failed to read program source from {}: {}", fp, e)),
    }
}
