use actix_multipart::Multipart;
use actix_web::{HttpRequest, HttpResponse};
use async_std::prelude::*;
use futures::{StreamExt, TryStreamExt};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use sanitize_filename::sanitize;
use std::{env, io};

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

    let rand_stuff: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(4)
        .map(char::from)
        .collect();

    let mut filename = String::from("");
    while let Ok(Some(mut field)) = payload.try_next().await {
        filename = format!("{}-{}", rand_stuff, sanitize(field.content_disposition().unwrap().get_filename().unwrap()));
        let filepath = format!("{}{}", ROOT_DIR, filename);
        let mut f = async_std::fs::File::create(filepath).await?;

        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).await?;
        }
    }
    Ok(HttpResponse::Ok().body(format!("https://f.weirdnatto.in/{}", filename)))
}
