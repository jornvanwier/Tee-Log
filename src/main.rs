extern crate chrono;
extern crate clap;

use chrono::prelude::*;
use clap::{App, ArgMatches};
use std::fs::{File, OpenOptions};
use std::io::{self, BufWriter};
use std::io::prelude::*;
use std::fmt::Display;

const DEFAULT_FILENAME: &str = "log.txt";

fn main() {
    let matches = get_arguments();

    let file = get_file(&matches);
    let mut file = BufWriter::new(file);

    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    // Initialize a writer for stdout, only used when print is true
    let stdout = io::stdout();
    let stdout = stdout.lock();
    let mut stdout = BufWriter::new(stdout);

    let print = matches.is_present("print");
    let timestamp = !matches.is_present("notimestamp");

    let mut buf: String = String::new();

    write(format!("Start log at {}\n", get_timestamp()),
          &mut file,
          &mut stdout,
          print);

    while stdin.read_line(&mut buf).expect("Couldn't read from stdin") > 0 {
        let line = if timestamp {
            format!("[{}] {}", get_timestamp(), buf)
        } else {
            format!("{}", buf)
        };

        write(line, &mut file, &mut stdout, print);

        buf.clear();
    }
}

fn write<S: Display, WF: Write, WO: Write>(line: S,
                                           file: &mut BufWriter<WF>,
                                           stdout: &mut BufWriter<WO>,
                                           print: bool) {
    let _ = write!(file, "{}", line);

    if print {
        let _ = write!(stdout, "{}", line);
    }
}

fn get_timestamp() -> String {
    return Local::now().format("%F %T:0%.3f").to_string();
}

fn get_arguments() -> ArgMatches<'static> {
    App::new("Tee Log")
        .version("0.1")
        .author("Jorn van Wier <jornvanwier@gmail.com>")
        .about("Reads the output of a program and outputs it to a file with an added timestamp")
        .args_from_usage("
            [file]          -f, --file <FILE>   'Sets the file to output to'
            [print]         -p, --print         'Print to stdout'
            [notimestamp]   -n, --notimestamp   'Don't add a timestamp to the messages'
        ")
        .get_matches()
}

fn get_file(matches: &ArgMatches) -> File {
    let file = matches.value_of("file").unwrap_or(DEFAULT_FILENAME);
    OpenOptions::new()
        .read(false)
        .append(true)
        .create(true)
        .open(file)
        .expect("Could not open log file")
}
