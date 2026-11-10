use std::{io, string::FromUtf8Error};

use serde_json::Value;
use thiserror::Error;
use tokio::{fs::File, io::AsyncReadExt};

#[allow(unused)]
async fn read_and_parse(file_name: &str) -> String {
    let mut f = File::open(file_name).await.unwrap();
    let mut buf = Vec::new();
    f.read_buf(&mut buf).await.unwrap();
    let content = String::from_utf8(buf).unwrap();
    let json_content: Value = serde_json::from_str(content.as_str()).unwrap();
    let greeting = json_content.get("message").unwrap().to_string();
    greeting
}

#[derive(Error, Debug)]
pub enum ReadAndParseError {
    #[error("Failed to access config file: {0:?}")]
    ConfigFileAccessError(#[from] io::Error),
    #[error("UTF-8 error")]
    Utf8Error(#[from] FromUtf8Error),
    #[error("JSON parse error")]
    JsonParseError(#[from] serde_json::Error),
    #[error("Missing 'message' field")]
    MissingMessageField,
}

async fn read_and_parse_v2(file_name: &str) -> Result<String, ReadAndParseError> {
    let mut f = File::open(file_name).await?;
    let mut buf = Vec::new();
    f.read_buf(&mut buf).await?;
    let content = String::from_utf8(buf)?;
    let json_content: Value = serde_json::from_str(content.as_str())?;
    let greeting = json_content.get("message").ok_or(ReadAndParseError::MissingMessageField)?.to_string();
    Ok(greeting)
}

async fn read_and_parse_v3(file_name: &str) -> anyhow::Result<String> {
    let mut f = File::open(file_name).await?;
    let mut buf = Vec::new();
    f.read_buf(&mut buf).await?;
    let content = String::from_utf8(buf)?;
    let json_content: Value = serde_json::from_str(content.as_str())?;
    let greeting = json_content.get("message").ok_or(ReadAndParseError::MissingMessageField)?.to_string();
    Ok(greeting)
}

#[tokio::main]
async fn main() -> Result<(), i32> {
    let greeting = read_and_parse_v2("dem.json").await;
    match greeting {
        Ok(msg) => println!("{}", msg),
        Err(e) => match e {
            ReadAndParseError::ConfigFileAccessError(err) => eprintln!("Config file access error: {:?}", err),
            ReadAndParseError::Utf8Error(err) => eprintln!("UTF-8 error: {:?}", err),
            ReadAndParseError::JsonParseError(err) => eprintln!("JSON parse error: {:?}", err),
            ReadAndParseError::MissingMessageField => eprintln!("Missing 'message' field"),
        },
    }

    let greeting = read_and_parse_v3("dem.json").await;
    match greeting {
        Ok(msg) => println!("{}", msg),
        Err(e) => if let Some(inner_error) = e.downcast_ref::<io::Error>() {
            eprintln!("IO error: {:?}", inner_error);
        } else if let Some(inner_error) = e.downcast_ref::<FromUtf8Error>() {
            eprintln!("UTF-8 error: {:?}", inner_error);
        } else if let Some(inner_error) = e.downcast_ref::<serde_json::Error>() {
            eprintln!("JSON parse error: {:?}", inner_error);
        } else {
            eprintln!("Error: {}", e);
        },
    }

    let second = read_and_parse_v2("demo.json")
        .await
        .map_err(|_| -1)?;
    let third = read_and_parse_v3("demo.json")
        .await
        .map_err(|_| -1)?;

    Ok(())
}
