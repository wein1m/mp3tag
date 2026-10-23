use id3::{ Error, ErrorKind, Tag, TagLike, Version };
use std::fs;

pub fn view(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let tag = match Tag::read_from_path(path) {
        Ok(tag) => tag,
        Err(Error{kind: ErrorKind::NoTag, ..}) => Tag::new(),
        Err(err) => return Err(Box::new(err))
    };

    println!("title: {}", tag.title().unwrap());
    println!("artist: {}", tag.artist().unwrap());

    Ok(())
}

pub fn edit_title(path: &str, new_title: &str) -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = std::env::temp_dir().join("temp.mp3");
    fs::copy(path, &temp_file)?;

    let mut tag = match Tag::read_from_path(&temp_file) {
        Ok(tag) => tag,
        Err(Error{kind: ErrorKind::NoTag, ..}) => Tag::new(),
        Err(err) => return Err(Box::new(err))
    };

    println!("Path: {}", temp_file.display());
    println!("title: {}", tag.title().unwrap());

    tag.set_title(new_title);

    println!("title: {}", tag.title().unwrap());

    tag.write_to_path(temp_file, Version::Id3v24)?;
    println!("Successfully edited!~");

    Ok(())
}
