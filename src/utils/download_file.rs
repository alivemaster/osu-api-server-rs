use std::error::Error;
use std::{
    fs::File,
    io::{Cursor, copy},
    path::PathBuf,
};

pub async fn download_file(url: &str, path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let res = reqwest::get(url).await?;
    let mut buffer = Cursor::new(res.bytes().await?);
    let mut file = File::create(path)?;
    copy(&mut buffer, &mut file)?;

    Ok(())
}
