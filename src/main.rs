use std::fmt::Debug;

use text_transformer;

#[derive(Debug, PartialEq, Eq)]
pub enum InputFile {
    InStream,
    Files(Vec<String>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ProgramSource {
    Literal(String),
    File(String),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    input_files: InputFile,
    program: ProgramSource,
}

fn main() {
    let config = match parse_args(std::env::args()) {
        Result::Ok(c) => c,
        Result::Err(e) => {
            eprintln!("{}", e);
            eprintln!(
                "Usage:\n\
                tt -f program-file [ -- ] file ...\n\
                tt [ -- ] program-source file ...\n\
                "
            );
            std::process::exit(1);
        }
    };

    println!("{}", text_transformer::msg());
    println!("{:?}", config);
}

pub fn parse_args(args: impl Iterator<Item = String>) -> Result<Config, String> {
    let mut config = Config {
        input_files: InputFile::InStream,
        program: ProgramSource::Literal(String::new()),
    };

    // Assuming the first argument is the program name; this seems to be just
    // a convention, quoting the Rust documentation:
    // The first element is traditionally the path of the executable, but it
    // can be set to arbitrary text, and might not even exist. This means this
    // property should not be relied upon for security purposes.
    // let mut args1 = args.skip(1);
    let mut args = args.skip(1).peekable();

    // First parse flags
    while let Some(arg) = args.peek() {
        match arg.as_str() {
            "-f" | "--file" => {
                let arg = arg.to_owned();
                args.next();
                let file_name = args.next().ok_or(format!("{} takes a parameter", arg))?;
                config.program = ProgramSource::File(file_name);
            }
            "--" => {
                args.next();
                break;
            }
            _ => break,
            // a => return Result::Err(format!("Unknown argument: '{}'", a)),
        };
    }

    // If -f hasn't been passed, the program source is the next argument
    config.program = match config.program {
        f @ ProgramSource::File(_) => f,
        ProgramSource::Literal(_) => match args.next() {
            None => Result::Err("Program source hasn't been specified")?,
            Some(source) => ProgramSource::Literal(source),
        },
    };

    // everything after is files
    match args.peek() {
        None => {
            config.input_files = InputFile::InStream;
        }
        Some(_) => {
            config.input_files = InputFile::Files(args.collect());
        }
    }

    Result::Ok(config)
}

#[cfg(test)]
mod tests {
    use super::{parse_args, Config, InputFile, ProgramSource};

    // Happy paths

    #[test]
    fn test_parse_args_flag() {
        let args = vec!["<program name>", "-f", "prog.tt"]
            .into_iter()
            .map(String::from);
        let actual = parse_args(args).unwrap();
        let expected = Config {
            input_files: InputFile::InStream,
            program: ProgramSource::File(String::from("prog.tt")),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_args_flag_dashes() {
        let args = vec!["<program name>", "-f", "prog.tt", "--"]
            .into_iter()
            .map(String::from);
        let actual = parse_args(args).unwrap();
        let expected = Config {
            input_files: InputFile::InStream,
            program: ProgramSource::File(String::from("prog.tt")),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_args_flag_file() {
        let args = vec!["<program name>", "-f", "prog.tt", "file"]
            .into_iter()
            .map(String::from);
        let actual = parse_args(args).unwrap();
        let expected = Config {
            input_files: InputFile::Files(vec![String::from("file")]),
            program: ProgramSource::File(String::from("prog.tt")),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_args_flag_dashes_file() {
        let args = vec!["<program name>", "-f", "prog.tt", "--", "file"]
            .into_iter()
            .map(String::from);
        let actual = parse_args(args).unwrap();
        let expected = Config {
            input_files: InputFile::Files(vec![String::from("file")]),
            program: ProgramSource::File(String::from("prog.tt")),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_args_program_source() {
        let args = vec!["<program name>", "some code"]
            .into_iter()
            .map(String::from);
        let actual = parse_args(args).unwrap();
        let expected = Config {
            input_files: InputFile::InStream,
            program: ProgramSource::Literal(String::from("some code")),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_args_dashes_program_source() {
        let args = vec!["<program name>", "--", "some code"]
            .into_iter()
            .map(String::from);
        let actual = parse_args(args).unwrap();
        let expected = Config {
            input_files: InputFile::InStream,
            program: ProgramSource::Literal(String::from("some code")),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_args_program_source_file() {
        let args = vec!["<program name>", "some code", "file"]
            .into_iter()
            .map(String::from);
        let actual = parse_args(args).unwrap();
        let expected = Config {
            input_files: InputFile::Files(vec![String::from("file")]),
            program: ProgramSource::Literal(String::from("some code")),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_args_dashes_program_source_file() {
        let args = vec!["<program name>", "--", "some code", "file"]
            .into_iter()
            .map(String::from);
        let actual = parse_args(args).unwrap();
        let expected = Config {
            input_files: InputFile::Files(vec![String::from("file")]),
            program: ProgramSource::Literal(String::from("some code")),
        };
        assert_eq!(actual, expected);
    }

    // Sad paths

    #[test]
    fn test_parse_args_missing_flag_argument() {
        let args = vec!["<program name>", "-f"].into_iter().map(String::from);
        let actual = parse_args(args);
        assert_eq!(actual, Result::Err(String::from("-f takes a parameter")));
    }
}
