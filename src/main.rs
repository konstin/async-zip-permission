use anyhow::Result;
use async_zip::tokio::write::ZipFileWriter;
use async_zip::{Compression, ZipEntryBuilder, ZipString};

#[tokio::main]
async fn main() -> Result<()> {
    let wheel_file = tokio::fs::File::create("data.zip").await?;
    let mut wheel_writer = ZipFileWriter::with_tokio(wheel_file);
    wheel_writer
        .write_entry_whole(
            ZipEntryBuilder::new(ZipString::from("Readme.txt"), Compression::Deflate),
            "hello world".as_bytes(),
        )
        .await?;
    wheel_writer.close().await?;
    Ok(())
}
