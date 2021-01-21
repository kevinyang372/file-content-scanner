extern crate clap;

use clap::App;
use std::io;

use fc::{parse_args, run};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let stdout = io::stdout();
    let handle = io::BufWriter::new(stdout);
    let matches = App::new("fc")
                        .version("1.0")
                        .author("Kevin Y. <yunfan.yang.kevin@gmail.com>")
                        .about("Display file content in command line interface")
                        .args_from_usage(
                            "<INPUT>            'Sets the input file to use'
                            -t, --tail=[TAIL]   'Sets the tail number'
                            -h, --head=[HEAD]   'Sets the head number'")
                        .get_matches();
    
    let configs = parse_args(&matches);
    run(configs, handle)
}
