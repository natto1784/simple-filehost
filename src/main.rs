mod post;
use actix_files::NamedFile;
use actix_web::{web, App, HttpRequest, HttpServer};
use std::{io, path::PathBuf};

const ROOT_DIR: &str = "/files/";

async fn get_file(req: HttpRequest) -> Result<NamedFile,io::Error> {
    let file_name = req.match_info().get("file").unwrap();
    let file_path: PathBuf = format!("{}{}", ROOT_DIR, file_name).parse().unwrap();
    match NamedFile::open(file_path) {
        Ok(file) => Ok(file.disable_content_disposition()),
        Err(_) => Err(io::Error::new(
            io::ErrorKind::Other,
            "No such file there, bhay",
        )),
    }
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/{file}", web::get().to(get_file))
            .route("/", web::post().to(post::post))
    })
    .bind(("0.0.0.0", 8888))?
    .run()
    .await
}
