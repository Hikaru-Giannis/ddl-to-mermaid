mod sql_to_mermaid;
use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {

    // ファイルを読み込む
    let input = fs::read_to_string("input.sql")?;

    // 入力を解析し、mermaidのコードを生成
    let output = sql_to_mermaid::convert(&input);

    // 出力
    println!("{}", output);

    Ok(())
}
