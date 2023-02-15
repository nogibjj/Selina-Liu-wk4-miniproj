/* An Actix webservice that has two routes
A. / - returns a hello world
B. /flower - returns a random flower
 */

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
// use std::path::PathBuf;
// use actix_files::NamedFile;

// import the random_flower function from lib.rs
use webflower::random_flower;

// create a function that returns a hello world
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world Random Flowers!")
}

// async fn img_response(x: PathBuf) -> HttpResponse {
//     let file = actix_files::NamedFile::open_async(x).await.unwrap();
//     file.into_response()
// }

// create a function that returns a random image
// #[get("/flower")]
// async fn flower() -> impl Responder {
//     let flower = random_flower();
//     let flower_image = format!("../static/{}.jpg", flower);
//     let flower_image = PathBuf::from(flower_image);
//     img_response(flower_image).await
// }

// create a function that returns a random flower
#[get("/flower")]
async fn flower() -> impl Responder {
    let flower = random_flower();
    HttpResponse::Ok().body(flower)
}

// create a function that returns a random flower image from static/
// #[get("/flower")]
// async fn flower() -> impl Responder {
//     let flower = random_flower();
//     // write a function that matches flower name to a string literal of its path
//     let flower_image = match flower.as_str() {
//         "Rose" => "../static/Rose.jpg",
//         "CherryBlossom" => "../static/CherryBlossom.jpg",
//         "Daisy" => "../static/Daisy.jpg",
//         "Lotus" => "../static/Lotus.jpg",
//         "Sunflower" => "../static/Sunflower.jpg",
//         _ => "../static/Rose.jpg",
//     };
//     let image_content = include_bytes!(flower_image);
//     // let flower_image = format!("../static/{}.jpg", flower);
//     // let flower_image = PathBuf::from(flower_image);
//     HttpResponse::Ok()
//         .content_type("image/jpg")
//         .body(image_content.to_vec())
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(flower))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
