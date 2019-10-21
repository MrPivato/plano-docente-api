use rocket_include_static_resources::StaticResponse;


pub fn favicon_dir() -> std::string::String {
	"src/static_content/favicon.ico".to_string()
}

pub fn index_dir() -> std::string::String {
	"src/static_content/index.html".to_string()
}

#[get("/")]
pub fn index() -> StaticResponse {
    static_response!("index_page")
}

#[get("/favicon.ico")]
pub fn favicon() -> StaticResponse {
    static_response!("favicon")
}