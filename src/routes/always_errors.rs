use axum::{http::StatusCode, response::{Response, IntoResponse}};

pub async fn always_errors()->Result<(),StatusCode>{

  Err(StatusCode::IM_A_TEAPOT)
}


pub async fn returns_201()-> Response{
  (
    StatusCode::CREATED,
    "This is a 201"
  ).into_response()
}

