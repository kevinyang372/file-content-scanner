extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let matches = App::new("File content scanner")
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
    let tail_number = matches.value_of("tail").unwrap_or("-1");
    let head_number = matches.value_of("head").unwrap_or("-1");
    println!("Input value: {} {} {}", input_file, tail_number, head_number);

    let content = std::fs::read_to_string(&input_file)?;
    let mut i = 1;

    for line in content.lines() {
        println!("Line {}: {}", i, line);
        i += 1;
    }

    Ok(())
}
