use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use actix_web::{HttpRequest, HttpResponse};
use async_std::prelude::*;
use sanitize_filename::sanitize;
use serde::Serialize;
use std::{env, io};

#[derive(Serialize)]
struct ReturnData {
    file: String,
    host: String,
    protocol: String,
    url: String,
}

impl ReturnData {
    fn new(_file:&str, _host: &str, _protocol: &str) -> Self {
        Self {
            file: String::from(_file),
            host: String::from(_host),
            protocol: String::from(_protocol),
            url: format!("{}://{}/{}", _protocol, _host, _file),
        }
    }
}

const ROOT_DIR: &'static str = "/files/";

fn get_token() -> String {
    env::var("FILEHOST_KEY").expect("FILEHOST_KEY is not set")
}

pub async fn post(req: HttpRequest, mut payload: Multipart) -> Result<HttpResponse, io::Error> {
    let key_header = req.headers().get("key");

    if key_header.is_none() {
        return Ok(HttpResponse::Unauthorized().body("No key provided"));
    }

    let key_val = key_header.unwrap().to_str().unwrap();

    if key_val != get_token() {
        return Ok(HttpResponse::Unauthorized().body("Invalid key provided"));
    }

    let mut filename = String::from("");
    while let Ok(Some(mut field)) = payload.try_next().await {
        filename = sanitize(
            field.content_disposition().unwrap().get_filename().unwrap(),
        );
        let filepath = format!("{}{}", ROOT_DIR, filename);
        let mut f = async_std::fs::File::create(filepath).await?;

        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).await?;
        }
    }
    Ok(HttpResponse::Ok().body(serde_json::to_string(&ReturnData::new(&filename, "f.weirdnatto.in", "https")).unwrap()))
}
