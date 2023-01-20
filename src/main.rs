
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
    <html>
        <head>
            <title>Vader - the Grader</title>
        </head>
        <body>
            <h2>Vader - the Grader</h2>
        </body>
    </html>
    ")
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/",routes![index,grade])
        .mount("/static", FileServer::from(relative!("static")))
}