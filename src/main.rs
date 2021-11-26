use actix_web::{web, App, Error, HttpResponse, HttpServer};
use std::fs;

async fn index() -> Result<HttpResponse, Error> {
    let contents = fs::read_to_string("./templates/index.html").unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(contents))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::resource("/").to(index)))
        .bind("localhost:3000")
        .expect("Can not bind to port 3000")
        .run()
        .await
}
