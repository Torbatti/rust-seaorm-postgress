use axum::{extract::Query, Json};
use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize)]
pub struct QueryParams{
  tags: String,
  id: i32,
}

pub async fn apps_by_tags(Query(query) : Query<QueryParams>) -> Json<QueryParams>{
  Json(query)
}