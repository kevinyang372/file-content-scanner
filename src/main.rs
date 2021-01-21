extern crate clap;

use clap::App;
use std::io;

use fc::{parse_args, run};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let stdout = io::stdout();
    let handle = io::BufWriter::new(stdout);

    // Define the list of valid arguments
    let matches = App::new("fc")
                        .version("1.0")
                        .author("Kevin Y. <yunfan.yang.kevin@gmail.com>")
                        .about("Display file content in command line interface")
                        .args_from_usage(
                            "<INPUT>            'Sets the input file to use'
                            -t, --tail=[TAIL]   'Sets the tail number'
                            -h, --head=[HEAD]   'Sets the head number'")
                        .get_matches();
    
    // Parse user inputs into arguments and run the main logic
    let configs = parse_args(&matches);
    run(configs, handle)
}
