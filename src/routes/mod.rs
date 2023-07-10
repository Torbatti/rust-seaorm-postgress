use axum::{Router, routing::{get, post}, body::Body};

mod route_app;
use route_app::mirror_body_string;

mod route_root;
use route_root::root_handler;

mod route_json_parser;
use route_json_parser::mirror_body_json;

pub fn create_routes() -> Router<Body>{
  Router::new()
    .route("/", get(root_handler))
    .route("/app/:id", post(mirror_body_string))
    .route("/json_mirror", post(mirror_body_json))
}
