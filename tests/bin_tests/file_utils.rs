// 2025 Steven Chiacchira
use std::path::Path;

pub fn file_contents_equal(file_1: &Path, file_2: &Path) -> Result<bool, std::io::Error> {
    if !file_1.is_file() || !file_2.is_file() {
        return Ok(file_1.is_file() && file_2.is_file());
    }

    let bytes_1 = std::fs::read(file_1)?;
    let bytes_2 = std::fs::read(file_2)?;

    Ok(bytes_1 == bytes_2)
}
