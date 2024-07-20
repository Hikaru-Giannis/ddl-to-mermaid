mod sql_to_mermaid;

fn main() {

    // TODO 文字列を直書き
    let input = "CREATE TABLE users (id INT, name TEXT, age INT)";

    // 入力を解析し、mermaidのコードを生成
    let output = sql_to_mermaid::convert(input);

    // 出力
    println!("{}", output);
}
