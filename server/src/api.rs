use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub data: T,
    pub message: String,
}

impl<T> ApiResponse<T> {
  pub fn success(data: T) -> Self {
    ApiResponse {
      status: "success".to_string(),
      data: data,
      message: "".to_string(),
    }
  }
}
