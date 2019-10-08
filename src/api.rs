use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub data: Option<T>,
    pub message: String,
}

impl<T> ApiResponse<T> {
  pub fn success(data: T) -> Self {
    ApiResponse {
      status: "success".to_string(),
      data: Some(data),
      message: "".to_string(),
    }
  }

  pub fn error() -> Self {
    ApiResponse {
      status: "error".to_string(),
      data: None,
      message: "".to_string(),
    }
  }
}
