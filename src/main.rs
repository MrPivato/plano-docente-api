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

/* --- Models --- */
pub mod ciclos_letivos_model;
pub mod niveis_ensino_model;
pub mod professors_model;
pub mod users_model;
pub mod cursos_model;
pub mod componentes_curriculares_model;
pub mod atendimentos_model;
pub mod atividades_administrativas_model;


/* --- Routes --- */
pub mod ciclos_letivos_route;
pub mod default_routes;
pub mod niveis_ensino_route;
pub mod professors_route;
pub mod users_route;
pub mod cursos_route;
pub mod componentes_curriculares_route;
pub mod atendimentos_route;
pub mod atividades_administrativas_route;


#[database("PDC")]
pub struct DbConn(diesel::MysqlConnection);

fn main() {
    rocket::ignite()
        .attach(StaticResponse::fairing(|resources| {
            static_resources_initialize!(
                resources,
                "favicon",
                default_routes::favicon_dir(),
                "index_page",
                default_routes::index_dir(),
            );
        }))
        .mount(
            "/",
            routes![
                // default routes
                default_routes::index,
                default_routes::favicon,

                // users routes
                users_route::create_user,
                users_route::read_user,
                users_route::read_users,
                users_route::update_user,
                users_route::delete_user,

                // professors routes
                professors_route::create_professor,
                professors_route::read_professor,
                professors_route::read_professors,
                professors_route::update_professor,
                professors_route::delete_professor,

                // niveis_ensino routes
                niveis_ensino_route::create_niveis_ensino,
                niveis_ensino_route::read_niveis_ensino_unique,
                niveis_ensino_route::read_niveis_ensino,
                niveis_ensino_route::update_niveis_ensino,
                niveis_ensino_route::delete_niveis_ensino,

                // ciclos_letivos routes
                ciclos_letivos_route::create_ciclos_letivos,
                ciclos_letivos_route::read_ciclos_letivos_unique,
                ciclos_letivos_route::read_ciclos_letivos,
                ciclos_letivos_route::update_ciclos_letivos,
                ciclos_letivos_route::delete_ciclos_letivos,

                // cursos routes
                cursos_route::create_curso,
                cursos_route::read_curso,
                cursos_route::read_cursos,
                cursos_route::update_curso,
                cursos_route::delete_curso,

                // componentes_curriculares routes
                componentes_curriculares_route::create_componentes_curriculares,
                componentes_curriculares_route::read_componentes_curriculares_unique,
                componentes_curriculares_route::read_componentes_curriculares,
                componentes_curriculares_route::update_componentes_curriculares,
                componentes_curriculares_route::delete_componentes_curriculares,

                // atendimentos routes
                atendimentos_route::create_atendimentos,
                atendimentos_route::read_atendimentos,

                // atividades_administrativas routes
                atividades_administrativas_route::create_atividades_administrativas,
                atividades_administrativas_route::read_atividades_administrativas,
            ],
        )
        .attach(DbConn::fairing())
        .launch();
}
