use axum::{TypedHeader, headers::UserAgent, http::HeaderMap};

pub async fn mirror_user_agent(TypedHeader(user_agent):TypedHeader<UserAgent>) -> String{
  user_agent.to_string()
}

pub async fn mirror_custom_header(headers:HeaderMap) -> String{
  let message_value = headers.get("x-message").unwrap();
  let message = message_value.to_str().unwrap().to_owned();
  message
}