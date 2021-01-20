extern crate clap;
use clap::App;
use std::collections::VecDeque;
use std::io;


fn scan_file(content: &str, mut handle: impl std::io::Write,head_number: &usize, tail_number: &usize) -> Result<(), Box<dyn std::error::Error>> {
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

    Ok(())
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);
    let matches = App::new("fc")
                        .version("1.0")
                        .author("Kevin Y. <yunfan.yang.kevin@gmail.com>")
                        .about("Display file content in command line interface")
                        .args_from_usage(
                            "<INPUT>            'Sets the input file to use'
                            -t, --tail=[TAIL]   'Sets the tail number'
                            -h, --head=[HEAD]   'Sets the head number'")
                        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let input_file = matches.value_of("INPUT").unwrap();
    let tail_number = matches.value_of("tail").unwrap_or("0").parse::<usize>().unwrap();
    let head_number = matches.value_of("head").unwrap_or("0").parse::<usize>().unwrap();

    let content = std::fs::read_to_string(&input_file)?;

    scan_file(&content, &mut handle, &head_number, &tail_number)
}
