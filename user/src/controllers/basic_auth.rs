use crate::graphql::types::CustomError;
use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};

#[derive(GraphQLInputObject, serde::Deserialize, serde::Serialize)]
pub struct RegisterInput {
    username: String,
    email: String,
    password: String,
}

#[derive(GraphQLObject, serde::Deserialize, serde::Serialize)]
pub struct RegisterResponse {
    pub username: String,
}

impl Responder for RegisterResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

pub type RegisterOutput = Result<RegisterResponse, CustomError>;

pub fn register_controller(input: RegisterInput) -> Result<RegisterResponse, CustomError> {
    Ok(RegisterResponse {
        username: "".to_string(),
    })
}
