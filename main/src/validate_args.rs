pub fn validate_args(args: Vec<String>) -> Result<(), String> {
    if args.len() != 2 {
        return Err("引数の数が正しくありません".to_string());
    }
    Ok(())
}
