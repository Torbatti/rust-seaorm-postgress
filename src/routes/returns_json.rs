use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct RandomData{
  username:String,
  message:String,
  count:i32,
}
pub async fn returns_json() -> Json<RandomData>{
  let data = RandomData{
    username: "YoYO1".to_owned(),
    message: "im a random message".to_owned(),
    count: 322 
  };
  Json(data)
}