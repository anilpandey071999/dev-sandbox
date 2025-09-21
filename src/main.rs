use tokio::fs::{self as tokio_fs, DirEntry};
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut entries = tokio_fs::read_dir("fake-logs/app/").await?;
    let mut files = Vec::new();
    while let Some(entry) = entries.next_entry().await?{
        let file_name = entry.file_name().to_string_lossy().into_owned();
        println!("file {}", file_name);
        files.push(file_name);
    }
    
    Ok(())
}
