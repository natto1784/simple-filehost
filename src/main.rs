mod post;
use actix_files::NamedFile;
use actix_web::{web, App, HttpRequest, HttpServer, HttpResponse};
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

async fn index() -> Result<HttpResponse, io::Error> {
    let html = 
    "<html>
        <body>
            Use curl to upload: <br> curl -F file=@\"[file]\" --header \"key: [key]\"  https://f.weirdnatto.in
            <br> Replace [file] with your local file and contact natto#5209 or natto#1264 on discord for [key]
            <br> An URL will be returned after the upload, use that to access your content
            <br> Content will be deleted by me roughly every 7 days
        </body>
    </html>";
    Ok(HttpResponse::Ok().body(html))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/{file}", web::get().to(get_file))
            .route("/", web::post().to(post::post))
            .route("/", web::get().to(index))
    })
    .bind(("0.0.0.0", 8888))?
    .run()
    .await
}
