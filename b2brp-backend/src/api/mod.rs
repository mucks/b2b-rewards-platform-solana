mod dto;

use self::dto::{LoginDto, SignUpDto};
use crate::db::{models_create::CreateUser, DbConn, DbPool};
use actix_web::{error::ErrorInternalServerError, web, FromRequest};
use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine as _};
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use hmac::digest::KeyInit;
use hmac::Hmac;
use jwt::SignWithKey;
use rand::Rng;
use sha2::Sha256;
use std::collections::BTreeMap;
use std::future::{ready, Ready};

pub struct Api {
    conn: DbConn,
}

impl Api {
    pub fn new(pool: web::Data<DbPool>) -> Result<Self> {
        Ok(Self { conn: pool.get()? })
    }

    fn hash_password(password: &str, salt: &[u8; 32]) -> Result<String> {
        let config = argon2::Config::default();
        let pw = argon2::hash_encoded(password.as_bytes(), salt, &config)?;
        Ok(pw)
    }

    fn verify_password(password: &str, hash: &str, salt: &[u8]) -> Result<bool> {
        let is_valid = argon2::verify_encoded_ext(hash, password.as_bytes(), salt, &[])?;
        Ok(is_valid)
    }

    pub fn login(&mut self, dto: LoginDto) -> Result<String> {
        let user = crate::schema::User::table
            .filter(crate::schema::User::email.eq(dto.email))
            .first::<crate::db::models::User>(&mut self.conn)?;

        let salt = general_purpose::STANDARD_NO_PAD.decode(user.salt.as_bytes())?;
        let is_valid = Self::verify_password(&dto.password, &user.password, &salt)?;

        if !is_valid {
            return Err(anyhow!("Invalid password"));
        }

        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET NOT SET!");
        let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes())?;
        let mut claims = BTreeMap::new();
        claims.insert("user_id", user.id.to_string());

        let token_str = claims.sign_with_key(&key)?;
        Ok(token_str)
    }

    pub fn sign_up(&mut self, dto: &SignUpDto) -> Result<()> {
        let salt: [u8; 32] = rand::thread_rng().gen();
        let hashed_pw = Self::hash_password(&dto.password, &salt)?;

        let create_user = CreateUser {
            email: dto.email.clone(),
            name: dto.name.clone(),
            password: hashed_pw,
            salt: general_purpose::STANDARD_NO_PAD.encode(salt),
        };

        diesel::insert_into(crate::schema::User::table)
            .values(&create_user)
            .execute(&mut self.conn)?;

        Ok(())
    }
}

impl FromRequest for Api {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        ErrorInternalServerError("test");
        let pool = match req.app_data::<web::Data<DbPool>>() {
            Some(p) => p,
            None => {
                return ready(Err(ErrorInternalServerError(
                    "could not connect to database",
                )));
            }
        };

        let api = Api::new(pool.clone())
            .map_err(|e| ErrorInternalServerError(format!("could not connect to database: {}", e)));

        ready(api)
    }
}
