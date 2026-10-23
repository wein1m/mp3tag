use id3::{ Tag, TagLike };

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tag = Tag::read_from_path("assets/test.mp3")?;

    if let Some(artist) = tag.artist() {
        println!("artist: {}", artist);
    }

    if let Some(title) = tag.title() {
        println!("title: {}", title);
    }

    Ok(())
}
