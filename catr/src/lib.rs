use std::error::Error;
use clap::{Command, crate_version, Arg, ArgAction};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config{
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn printer(buffer: Box<dyn BufRead>, number_lines: bool, number_nonblank_lines: bool) {
    let mut line_count = 1;
    for line in buffer.lines() {
        if number_lines {
            println!("{:>6}\t{}", line_count, line.unwrap());
            line_count += 1;
        }
        else if number_nonblank_lines {
            let s: String = line.unwrap();
            if s == "" {
                println!("");
            }
            else {
                println!("{:>6}\t{}", line_count, s);
                line_count += 1;
            }
        }
        else {
            println!("{}", line.unwrap());
        }
    }
}

pub fn run(config: Config) -> MyResult<()> {
    //dbg!(&config);

    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(buffy) => printer(buffy, config.number_lines, config.number_nonblank_lines),
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let _matches = Command::new("catr")
        .version(crate_version!())
        .author("alex")
        .about("Rust cat")
        .arg(
            Arg::new("number_lines")
                .short('n')
                .long("number")
                .help("number all output lines")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("number_nonblank_lines")
                .short('b')
                .long("number-nonblank")
                .help("number nonempty output lines")
                .conflicts_with("number_lines")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Concatenate FILE(s) to standard output")
                .default_value("-")
                .num_args(1..),
        )
        .get_matches();

    Ok(Config{
        files: _matches.get_many::<String>("files").unwrap().cloned().collect(),
        number_lines: _matches.get_flag("number_lines"),
        number_nonblank_lines: _matches.get_flag("number_nonblank_lines")
    })
}