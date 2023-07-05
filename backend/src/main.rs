#[macro_use]
extern crate rocket;

use dotenvy::dotenv;
use std::env;

use rocket::http::Method;
use rocket::{Build, Rocket};
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};

mod controllers;
use controllers::{recipe_controller, user_controller};

mod models;
mod database;
mod schema;
mod jwt;

#[cfg(test)]
mod tests;

#[launch]
fn rocket() -> Rocket<Build> {
    dotenv().ok();
    let frontend_url = env::var("FRONTEND_URL").expect("FRONTEND_URL must be set");

    let allowed_origins = AllowedOrigins::some_exact(&[frontend_url]);

    let cors = CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("CORS failed.");

    rocket::build()
        .mount(
            "/",
            routes![
                recipe_controller::recipe,
                recipe_controller::search,
                recipe_controller::single_recipe,
                recipe_controller::addrecipes,
                recipe_controller::delete,
                user_controller::login,
                user_controller::register,
                user_controller::profile
            ],
        )
        .attach(cors)
}
