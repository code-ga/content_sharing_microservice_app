use actix_web::web;

use crate::controllers::basic_auth::{register_controller, RegisterInput, RegisterOutput};
async fn login(input: web::Json<RegisterInput>) -> RegisterOutput {
    register_controller(input.0)
}
pub fn rest_scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/login").route(web::get().to(login)));
}
