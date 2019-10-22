#![feature(proc_macro_hygiene, decl_macro)]

extern crate chrono;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket_include_static_resources;

use rocket_include_static_resources::StaticResponse;

pub mod cors;
pub mod schema;

/* --- Project Models --- */
pub mod users_model;

/* --- Project Routes --- */
pub mod default_routes;
pub mod users_route;

#[database("PDC")]
pub struct DbConn(diesel::MysqlConnection);

fn main() {
    rocket::ignite()
        .attach(StaticResponse::fairing(|resources| {
            static_resources_initialize!(
                resources, 
                "favicon", default_routes::favicon_dir(),
                "index_page", default_routes::index_dir(),);
        }))
        .mount(
            "/",
            routes![
                default_routes::index,
                default_routes::favicon,
                users_route::create_user,
                users_route::read_user,
                users_route::read_users,
                users_route::update_user,
                users_route::delete_user,
            ],
        )
        .attach(DbConn::fairing())
        .launch();
}
