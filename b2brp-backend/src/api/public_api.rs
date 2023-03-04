use actix_web::{post, web::Json, HttpResponse, Responder, Scope};

use crate::{
    api::{
        dto::{LoginDto, SignUpDto},
        Api,
    },
    core::error::MyResult,
};

pub fn scope() -> Scope {
    Scope::new("/public-api").service(sign_up).service(login)
}

#[post("/login")]
pub async fn login(mut api: Api, dto: Json<LoginDto>) -> MyResult<impl Responder> {
    let token = api.login(dto.into_inner())?;
    Ok(Json(token))
}

#[post("/sign-up")]
pub async fn sign_up(mut api: Api, dto: Json<SignUpDto>) -> MyResult<impl Responder> {
    let login_dto = LoginDto {
        email: dto.email.clone(),
        password: dto.password.clone(),
    };

    api.sign_up(&dto.into_inner())?;

    let token = api.login(login_dto)?;
    Ok(Json(token))
}
