mod metadata;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    metadata::view("assets/test.mp3")?;
    metadata::edit_title("assets/test.mp3", "Happy! Lucky! Hardtek!")?;

    Ok(())
}
