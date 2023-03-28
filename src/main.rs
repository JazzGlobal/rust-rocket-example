#[macro_use] extern crate rocket;

use diesel::{QueryDsl, RunQueryDsl};
use rocket_dyn_templates::{context, Template};
use RocketPractice::{establish_connection};
use RocketPractice::models::{User};
use RocketPractice::schema::users::dsl::users;
use dotenvy::{dotenv, dotenv_override};

#[get("/")]
fn index() -> Template {
    let connection = &mut establish_connection();
    let results = users.limit(1).load::<User>(connection).expect("Error loading users");

    Template::render("index", context! {
        data: results,
    })
}



#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index]).attach(Template::fairing())
}