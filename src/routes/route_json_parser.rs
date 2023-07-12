use axum::Json;
use serde::{Serialize, Deserialize};

#[derive(Deserialize,Debug)]
pub struct MirrorJson{
  message: String,
}

#[derive(Serialize,Debug)]
pub struct MirrorJsonResponse{
  message: String,
  message_from_server: String,
}

pub async fn mirror_body_json(Json(body):Json<MirrorJson>) -> Json<MirrorJsonResponse> {
  Json(
    MirrorJsonResponse{
      message: body.message,
      message_from_server: "YOYO".to_owned(),
    }
  )  
}