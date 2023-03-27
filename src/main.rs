#[macro_use] extern crate rocket;

use rocket::serde::Serialize;
use rocket_dyn_templates::{context, Template};

#[derive(Serialize)]
struct Person {
    name: String,
    age: i32,
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {
        data: Person { name: String::from("chris"), age: 25 },
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index]).attach(Template::fairing())
}