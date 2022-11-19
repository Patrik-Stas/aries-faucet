use actix_web::{HttpResponse, ResponseError};
use log::error;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::application::error::BusinessError;

pub type RespResult = Result<HttpResponse, BusinessError>;

impl ResponseError for BusinessError {
    fn error_response(&self) -> HttpResponse {
        error!("An error occurred. {:?}", self);
        let resp = Resp::err(self.to_code(), &self.to_message());
        match self {
            BusinessError::ResourceNotFound(_) => HttpResponse::NotFound().json(resp),
            BusinessError::ValidationError(_) => HttpResponse::BadRequest().json(resp),
            BusinessError::ArgumentError => HttpResponse::BadRequest().json(resp),
            BusinessError::InternalError { .. } => HttpResponse::InternalServerError().json(resp)
        }
    }
}


#[derive(Deserialize, Serialize)]
pub struct Resp<T>
    where
        T: Serialize,
{
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

impl<T: Serialize> Resp<T> {
    pub fn ok(data: T) -> Self {
        Resp {
            code: 0, // todo: remove
            message: "ok".to_owned(), // todo: remove
            data: Some(data),
        }
    }

    pub fn to_json_result(&self) -> Result<HttpResponse, BusinessError> {
        Ok(HttpResponse::Ok().json(&self.data))
    }
}

impl Resp<()> {
    pub fn err(error: i32, message: &str) -> Self {
        Resp {
            code: error,
            message: message.to_owned(),
            data: None,
        }
    }
}