use serde_json::Value;
use tokio::{fs::File, io::AsyncReadExt};

async fn read_and_parse(file_name: &str) -> String {
    let mut f = File::open(file_name).await.unwrap();
    let mut buf = Vec::new();
    f.read_buf(&mut buf).await.unwrap();
    let content = String::from_utf8(buf).unwrap();
    let json_content: Value = serde_json::from_str(content.as_str()).unwrap();
    let greeting = json_content.get("message").unwrap().to_string();
    greeting
}

#[tokio::main]
async fn main() {
    let greeting = read_and_parse("demo.json").await;
    println!("{}", greeting);
}
