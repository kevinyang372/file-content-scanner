//! # file-content-scanner
//! **file-content-scanner** is a tool for users to quickly scan file contents under a
//! directory with better control. It offers optional arguments including head_number
//! and tail_number to selectively output information.

use clap::ArgMatches;
use std::fs;
use std::collections::VecDeque;

/// Representation of the user-input arguments
pub struct Config<'a> {
    // File name for scanning
    input_file: &'a str,
    /// Number of lines to keep counting from head
    head_number: usize,
    /// Number of lines to keep counting from tail
    tail_number: usize
}

impl<'a> Config<'a> {
    /// Create a new argument list
    fn new(input_file: &'a str, head_number: usize, tail_number: usize) -> Config<'a> {
        Config { input_file, head_number, tail_number }
    }
}

/// Main logic of outputing the file content
pub fn run(config: Config, mut handle: impl std::io::Write) -> Result<(), Box<dyn std::error::Error>>{

    // Special case when user used . as input file name
    if config.input_file == "." {
        for entry in fs::read_dir(".")? {
            let entry = entry?;
            let path = entry.path();

            // Filter out cases when the name is a directory
            if !path.is_dir() {
                scan_file(path.to_str().unwrap(), &mut handle, &config.head_number, &config.tail_number)?;
            }
        }
        
        return Ok(());
    }

    scan_file(config.input_file, &mut handle, &config.head_number, &config.tail_number)
}

/// Parse arguments into configurations
pub fn parse_args<'a>(matches: &'a ArgMatches) -> Config<'a> {
    let input_file = matches.value_of("INPUT").unwrap();
    let tail_number = matches.value_of("tail").unwrap_or("0").parse::<usize>().unwrap();
    let head_number = matches.value_of("head").unwrap_or("0").parse::<usize>().unwrap();

    Config::new(input_file, head_number, tail_number)
}

/// Scan and output the content of one individual file
fn scan_file(input_file: &str, mut handle: impl std::io::Write, head_number: &usize, tail_number: &usize) -> Result<(), Box<dyn std::error::Error>> {

    let content = fs::read_to_string(input_file)?;
    writeln!(handle, "{}", input_file)?;

    // Tracking the current line number
    let mut i = 1;

    // A queue for keeping a fixed number of lines
    let mut tail_queue = VecDeque::new();

    let head_number = *head_number;
    let tail_number = *tail_number;

    if head_number > 0 {
        writeln!(handle, "[HEAD]")?;
    }

    for line in content.lines() {
        if i <= head_number || (head_number == 0 && tail_number == 0) {
            writeln!(handle, "Line {}: {}", i, line)?;
        }

        if tail_number > 0 && tail_queue.len() == tail_number {
            tail_queue.pop_front();
        }

        if tail_number > 0 {
            tail_queue.push_back(line);
        }

        i += 1; 
    }

    if tail_number > 0 {
        writeln!(handle, "[TAIL]")?;
        for line in tail_queue {
            writeln!(handle, "Line {}: {}", i - tail_number, line)?;
            i += 1;
        }
    }

    writeln!(handle, "\n")?;
    Ok(())
}
