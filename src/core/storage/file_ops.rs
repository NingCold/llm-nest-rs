use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;

use anyhow::{Context, Result};

pub fn atomic_write(target: &Path, data: &[u8]) -> Result<()> {
    let dir = target.parent().context("Target has no parent directory")?;

    // Ensure the directory exists
    if !dir.exists() {
        fs::create_dir_all(dir)
            .with_context(|| format!("Failed to create directory: {}", dir.display()))?;
    }

    // Write directly to the target file
    let file = File::create(target)
        .with_context(|| format!("Failed to create file: {}", target.display()))?;
    let mut writer = BufWriter::new(file);
    writer.write_all(data)?;
    writer.flush()?;
    writer.get_ref().sync_all()?;

    Ok(())
}

pub fn check_disk_space(_path: &Path, _required_bytes: u64) -> bool {
    // TODO: Implement proper disk space check using statvfs
    true
}

pub fn delete_model_file(path: &Path) -> Result<()> {
    fs::remove_file(path)
        .with_context(|| format!("Failed to delete model file: {}", path.display()))?;
    Ok(())
}
