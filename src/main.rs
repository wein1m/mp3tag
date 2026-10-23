mod metadata;

use std::{ env, process, fs, path::PathBuf };
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

    let file_path = match matches.opt_str("f") {
        Some(f) => get_path(f.as_str()),
        None => {
            if let Some(path) = matches.free.first() {
                get_path(path)
            } else {
                eprintln!("{prog}: ERROR!! Song's filepath is a must!!!\n");
                print_usage(prog, &opts);
                process::exit(1);
            }
        }
    };

    metadata::view(&file_path)?;
    metadata::edit_title(&file_path, "Happy! Lucky! Hardtek!")?;

    Ok(())
}

fn print_usage(prog: &str, opts: &Options) {
    let brief = format!("Usage: {prog} [options]");
    print!("{}", opts.usage(&brief));
}

fn get_path(path: &str) -> PathBuf {
    let full_path = match fs::canonicalize(&path) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Uweehhh~~ I fwailed to get the full path TT. {e}");
            process::exit(1);
        }
    };

    PathBuf::from(full_path)
}
