use axum::extract::Path;

pub async fn mirror_body_string(Path(id): Path<i32>) -> String{
// pub async fn mirror_body_string(Path(id:String),body:String) -> String{
  id.to_string()
}