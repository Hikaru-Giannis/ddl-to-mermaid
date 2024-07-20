pub fn validate_extension(file_name: &str) -> Result<(), String> {
    if file_name.ends_with(".sql") {
        Ok(())
    } else {
        Err("拡張子が.sqlではありません".to_string())
    }
}
