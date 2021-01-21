use clap::ArgMatches;
use std::fs;
use std::collections::VecDeque;

pub struct Config<'a> {
    input_file: &'a str,
    head_number: usize,
    tail_number: usize
}

impl<'a> Config<'a> {
    fn new(input_file: &'a str, head_number: usize, tail_number: usize) -> Config<'a> {
        Config { input_file, head_number, tail_number }
    }
}

pub fn run(config: Config, mut handle: impl std::io::Write) -> Result<(), Box<dyn std::error::Error>>{
    if config.input_file == "." {
        for entry in fs::read_dir(".")? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_dir() {
                scan_file(path.to_str().unwrap(), &mut handle, &config.head_number, &config.tail_number)?;
            }
        }
        
        return Ok(());
    }

    scan_file(config.input_file, &mut handle, &config.head_number, &config.tail_number)
}

pub fn parse_args<'a>(matches: &'a ArgMatches) -> Config<'a> {
    let input_file = matches.value_of("INPUT").unwrap();
    let tail_number = matches.value_of("tail").unwrap_or("0").parse::<usize>().unwrap();
    let head_number = matches.value_of("head").unwrap_or("0").parse::<usize>().unwrap();

    Config::new(input_file, head_number, tail_number)
}

fn scan_file(input_file: &str, mut handle: impl std::io::Write, head_number: &usize, tail_number: &usize) -> Result<(), Box<dyn std::error::Error>> {

    let content = fs::read_to_string(input_file)?;
    writeln!(handle, "{}", input_file)?;

    let mut i = 1;
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
