use id3::{ Error, ErrorKind, Tag, TagLike, Version };
use std::{ fs, path::PathBuf };

pub enum MetadataField {
    Title,
    Artist
}

pub fn view(path: &PathBuf, field: MetadataField) -> Result<String, Box<dyn std::error::Error>> {
    let tag = match Tag::read_from_path(path) {
        Ok(tag) => tag,
        Err(Error{kind: ErrorKind::NoTag, ..}) => Tag::new(),
        Err(err) => return Err(Box::new(err))
    };

    let value = match field { 
        MetadataField::Title => tag.title(),
        MetadataField::Artist => tag.artist()
    };  

    Ok(value.unwrap_or("Unknown").to_string())
}

fn move_temp(path: &PathBuf) -> Result<PathBuf, std::io::Error> {
    let filename = path.file_name().unwrap();
    let temp_file = std::env::temp_dir().join(filename);
    fs::copy(path, &temp_file)?;

    Ok(temp_file)
}

pub fn edit_tag(path: &PathBuf, field: MetadataField, input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let temp_file = move_temp(path)?;

    let mut tag = match Tag::read_from_path(&temp_file) {
        Ok(tag) => tag,
        Err(Error{kind: ErrorKind::NoTag, ..}) => Tag::new(),
        Err(err) => return Err(Box::new(err))
    };

    match field {
        MetadataField::Title => tag.set_title(input),
        MetadataField::Artist => tag.set_artist(input),
    }

    tag.write_to_path(&temp_file, Version::Id3v24)?;

    fs::copy(&temp_file, path)?;
    fs::remove_file(temp_file)?;
    Ok(())
}
