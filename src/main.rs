mod metadata;

use std::{ env, process };
use getopts::Options;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let prog = &args[0];

    let mut opts = Options::new();
    opts.optflag("h", "help", "print usage and exit");
    opts.optflag("s", "show", "Show song's metadata");
    opts.optopt("f", "file", "Song to edit / view", "FILE");
    opts.optopt("a", "artist", "New artist name", "ARTIST");
    opts.optopt("t", "title", "New title name", "TITLE");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{prog}: Error: {e}");
            process::exit(1);
        }
    };

    if args.len() == 1 || matches.opt_present("h") {
        print_usage(prog, &opts);
        process::exit(0);
    }

    metadata::view("assets/test.mp3")?;
    metadata::edit_title("assets/test.mp3", "Happy! Lucky! Hardtek!")?;

    Ok(())
}

fn print_usage(prog: &str, opts: &Options) {
    let brief = format!("Usage: {prog} [options]");
    print!("{}", opts.usage(&brief));
}
