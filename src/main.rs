
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
            <link rel=\"stylesheet\" type=\"text/css\" href=\"/static/style.css\" />
        </head>
        <body>
            <div class=\"panel\">
                <h2><a href=\"/static/index.html\">Vader</a> - the Grader</h2>
            </div>
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