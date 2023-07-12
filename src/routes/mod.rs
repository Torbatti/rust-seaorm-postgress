use axum::{Router, routing::{get, post}, body::Body, http::Method, Extension, middleware};
use tower_http::cors::{CorsLayer, Any};

mod route_app;
use route_app::mirror_body_string;

mod route_apps;
use route_apps::apps_by_tags;

mod route_root;
use route_root::root_handler;

mod route_json_parser;
use route_json_parser::mirror_body_json;

mod headers_handler;
use headers_handler::mirror_user_agent;
use headers_handler::mirror_custom_header;

mod middleware_message;
use middleware_message::middleware_message;

mod read_middleware_custom_header;
use read_middleware_custom_header::read_middleware_custom_header;

mod set_middleware_custom_header;
use set_middleware_custom_header::set_middleware_custom_header;

mod always_errors;
use always_errors::always_errors;
use always_errors::returns_201;

mod returns_json;

#[derive(Clone)]
pub struct SharedData{
  pub message: String,
}

pub fn create_routes() -> Router<Body>{
  let cors = CorsLayer::new()
    .allow_methods([Method::GET,Method::POST])
    .allow_origin(Any);

  // !!! NOT THE BEST WAY TO SHARE MUTABLE DATAS !!!
  let shared_data = SharedData{
    message: "hello shared data".to_owned(),
  };

  Router::new()
    .route("/", get(root_handler))

    .route("/app/:id", post(mirror_body_string))
    .route("/developer/:id", post(mirror_body_string))
    .route("/publisher/:id", post(mirror_body_string))

    .route("/apps", post(mirror_body_string))
    .route("/apps/tags", post(apps_by_tags))

    .route("/json_mirror", post(mirror_body_json))

    .route("/mirror_user_agent", get(mirror_user_agent))
    .route("/mirror_custom_header", get(mirror_custom_header))

    .route("/middleware_message", get(middleware_message))

    .layer(Extension(shared_data))
    .layer(cors)

    .route("/read_middleware_custom_header", get(read_middleware_custom_header))
    .layer(middleware::from_fn(set_middleware_custom_header))

    .route("/always_errors", get(always_errors))
    .route("/returns_201", post(returns_201))

}
