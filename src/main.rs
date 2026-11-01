mod metadata;

use std::{ env, process, fs, path::PathBuf, io, io::Write };
use getopts::Options;

use crate::metadata::MetadataField;

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
                process::exit(1);
            }
        }
    };
    let new_title = matches.opt_str("t");
    let new_artist = matches.opt_str("a");

    if let Some(title) = &new_title {
        validate_data(title, "Title").unwrap_or_else(|e| {
            eprintln!("{prog}: ERRORRR {e}");
            process::exit(1);
        });
    }

    if let Some(artist) = &new_artist {
        validate_data(artist, "Artist").unwrap_or_else(|e| {
            eprintln!("{prog}: ERRORRR {e}");
            process::exit(1);
        });
    }

    if let Some(title) = &new_title {
        metadata::edit_tag(&file_path, MetadataField::Title, title.as_str())?;
        println!("Title has Successfully edited!~");
    }
    if let Some(artist) = &new_artist {
        metadata::edit_tag(&file_path, MetadataField::Artist, artist.as_str())?;
        println!("Artist has Successfully edited!~");
    }

    if new_title.is_none() && new_artist.is_none() {
        interactive_mode(&file_path, prog)?;
    }

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

fn validate_data(value: &str, field: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        return Err(format!("{field} cannot be empty!"));
    }

    Ok(())
}

fn interactive_mode(file_path: &PathBuf, prog: &str) -> Result<(), Box<dyn std::error::Error>> {
    let title = metadata::view(&file_path, MetadataField::Title).unwrap();
    let artist = metadata::view(&file_path, MetadataField::Artist).unwrap();

    println!("Original Title: {}", title);
    println!("Original Artist: {}", artist);

    println!("");

    print!("New Title: ");
    io::stdout().flush()?;
    
    let mut new_title = String::new();
    io::stdin().read_line(&mut new_title)?;

    if new_title.trim().is_empty() {
        new_title = title;
    }

    print!("New Artist: ");
    io::stdout().flush()?;

    let mut new_artist = String::new();
    io::stdin().read_line(&mut new_artist)?;

    if new_artist.trim().is_empty() {
        new_artist = artist;
    }

    println!("");

    print!("Save Changes: [Y/n] ");
    io::stdout().flush()?;

    let mut confirm = String::new();
    io::stdin().read_line(&mut confirm)?;

    let save_changes = match confirm.trim().to_lowercase().as_str() {
        "" | "y" | "yes" => true,
        "n" | "no" => false,
        _ => {
            eprintln!("{prog}: Only Accepting Y/N !!!");
            process::exit(1)
        }
    };

    if save_changes {
        metadata::edit_tag(&file_path, MetadataField::Title, new_title.trim())?;
        metadata::edit_tag(&file_path, MetadataField::Artist, new_artist.trim())?;
        println!("Successfully edited!~");
    } else {
        println!("Changes discarded..")
    };


    Ok(())
}
