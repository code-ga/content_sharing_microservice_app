use std::fmt::Display;

use actix_web::error;
use juniper::{FieldError, IntoFieldError, ScalarValue};

#[derive(GraphQLObject)]
pub struct LikeServiceQueryReturn {
    pub sdl: String,
}

#[derive(Debug)]
pub enum CustomError {
    Unauthorized,
}

impl<S: ScalarValue> IntoFieldError<S> for CustomError {
    fn into_field_error(self) -> FieldError<S> {
        match self {
            CustomError::Unauthorized => FieldError::new(
                "You are not authorized to access this resource",
                graphql_value!({
                    "type": "Unauthorized",
                    "message":"You are not authorized to access this resource"
                }),
            ),
        }
    }
}

impl CustomError {
    fn to_string_error(&self) -> String {
        match self {
            CustomError::Unauthorized => {
                "You are not authorized to access this resource".to_string()
            }
        }
    }
}
impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string_error())
    }
}

impl error::ResponseError for CustomError {}
