
use rocket::http::ContentType;
use rocket::fs::{FileServer,relative};

#[macro_use] extern crate rocket;

#[get("/")]
async fn index() -> (ContentType, &'static str) {
    (ContentType::HTML, "
    ")
}

#[post("/grade")]
async fn grade() -> (ContentType, &'static str) {
    (ContentType::HTML, "
    ")
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/",routes![index,grade])
        .mount("/static", FileServer::from(relative!("static")))
}