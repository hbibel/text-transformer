use std::fmt::Debug;
use std::fs;
use std::io;
use std::io::BufRead;
use std::io::Write;

use text_transformer::program;
use text_transformer::program_source;
use text_transformer::program_source::ProgramSource;

#[derive(Debug, PartialEq, Eq)]
pub enum InputFile {
    InStream,
    Files(Vec<String>),
}

pub enum Verbosity {
    Normal,
    Verbose,
    Spam,
}

pub struct Config {
    input_files: InputFile,
    program: ProgramSource,
    verbosity: Verbosity,
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

    env_logger::Builder::new()
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .filter_level(match config.verbosity {
            Verbosity::Normal => log::LevelFilter::Info,
            Verbosity::Verbose => log::LevelFilter::Debug,
            Verbosity::Spam => log::LevelFilter::Trace,
        })
        .init();

    let code = match program_source::code(config.program) {
        Result::Ok(code) => code,
        Result::Err(e) => {
            log::error!("{}", e);
            std::process::exit(1);
        }
    };
    let p = program::compile(&code).unwrap_or_else(|e| {
        log::error!("{}", e);
        std::process::exit(1);
    });
    let mut items = create_input_stream(config.input_files).unwrap();
    items
        .try_for_each(|input| p.run(&input).map(|out| println!("{}", out)))
        .unwrap_or_else(|err| {
            log::error!("{}", err);
            std::process::exit(1);
        });
}

struct Items {
    reader: Box<dyn BufRead>,
}

impl Iterator for Items {
    // TODO Using an iterator as abstraction for items means that we cannot
    // handle errors that occur during reading the input stream, as
    // Iterator::next returns an Option. I'm too lazy to change that now, but
    // I should fix it in future
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let delimiter = '\n' as u8;
        let mut buf = Vec::new();
        match self.reader.read_until(delimiter, &mut buf) {
            Err(_) => None,
            Ok(0) => None,
            Ok(_) => Some(String::from_utf8(buf).unwrap()),
        }
    }
}

fn create_input_stream(inf: InputFile) -> Result<Items, io::Error> {
    let reader: Box<dyn BufRead> = match inf {
        InputFile::InStream => Box::new(io::BufReader::new(io::stdin())),
        InputFile::Files(file_names) => {
            Box::new(io::BufReader::new(fs::File::open(&file_names[0])?))
        }
    };

    Ok(Items { reader })
}

fn parse_args(args: impl Iterator<Item = String>) -> Result<Config, String> {
    // TODO When adding more flags, consider using clap
    // TODO add -v, -vv, -vvv for verbosity

    let mut config = Config {
        input_files: InputFile::InStream,
        program: ProgramSource::Literal(String::new()),
        verbosity: Verbosity::Spam,
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
