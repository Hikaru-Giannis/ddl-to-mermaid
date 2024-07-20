mod sql_to_mermaid;
mod validate_args;
mod validate_extension;
use std::env;
use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {

    // 引数をCLIから取得
    let args: Vec<String> = env::args().collect();

    // 引数の数が正しいか検証
    validate_args::validate_args(args.clone())?;

    // ファイル名の拡張子が.sqlか検証
    validate_extension::validate_extension(&args[1])?;

    // ファイルを読み込む
    let input: String = fs::read_to_string(&args[1])?;

    // 入力を解析し、mermaidのコードを生成
    let output: String = sql_to_mermaid::convert(&input);

    // 出力
    println!("{}", output);

    Ok(())
}
