extern crate image;
extern crate getopts;

use getopts::{Matches, Options};
use std::error::Error;
use std::env;

fn print_usage(program : &str, opts : Options) {
    let brief = format!("Usage: {} SCENE_FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() -> Result<(), String> {
    run().map_err(|e| e.to_string())
}

fn run() -> Result<(), Box<dyn Error>> {
    let args : Vec<String> = env::args().collect();

    let mut opts : Options = Options::new();
    opts.optopt("o", "output", "output file name", "NAME");
    opts.optopt("i", "input", "input file name", "NAME");
    opts.optopt("s", "scale", "output resolution scale factor", "VALUE");
    opts.optopt("c", "cell_size", "output cell size", "VALUE");
    opts.optflag("h", "help", "print this help menu");
    let matches : Matches = opts.parse(&args[1..])?;

    if (matches.opt_present("h")) || (matches.free.is_empty()) {
        print_usage(&args[0], opts);
        return Ok(());
    }


    Ok(())
}