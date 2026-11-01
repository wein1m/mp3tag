use id3::{ Error, ErrorKind, Tag, TagLike, Version };
use std::{ fs, path::PathBuf };

pub fn view(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let tag = match Tag::read_from_path(path) {
        Ok(tag) => tag,
        Err(Error{kind: ErrorKind::NoTag, ..}) => Tag::new(),
        Err(err) => return Err(Box::new(err))
    };

    println!("title: {}", tag.title().unwrap());
    println!("artist: {}", tag.artist().unwrap());

    Ok(())
}

pub fn edit_title(path: &PathBuf, new_title: &str) -> Result<(), Box<dyn std::error::Error>> {
    let filename = path.file_name().unwrap();
    let temp_file = std::env::temp_dir().join(filename);
    fs::copy(path, &temp_file)?;

    let mut tag = match Tag::read_from_path(&temp_file) {
        Ok(tag) => tag,
        Err(Error{kind: ErrorKind::NoTag, ..}) => Tag::new(),
        Err(err) => return Err(Box::new(err))
    };

    println!("Temp Path: {}", temp_file.display());
    println!("Ori Path: {}", path.display());
    println!("");
    println!("Ori title: {}", tag.title().unwrap());

    tag.set_title(new_title);

    println!("New title: {}", tag.title().unwrap());

    tag.write_to_path(&temp_file, Version::Id3v24)?;

    fs::copy(&temp_file, path)?;
    fs::remove_file(temp_file)?;

    println!("Successfully edited!~");

    Ok(())
}
