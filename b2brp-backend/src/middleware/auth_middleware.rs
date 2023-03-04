use actix_service::Transform;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse},
    error::{Error, ErrorUnauthorized},
    http::header::{HeaderName, HeaderValue},
    web::Query,
    FromRequest, HttpMessage, HttpRequest,
};
use anyhow::anyhow;
use anyhow::Result;
use futures_util::{future::LocalBoxFuture, FutureExt};
use hmac::digest::KeyInit;
use hmac::Hmac;
use jwt::VerifyWithKey;
use sha2::Sha256;
use std::{
    collections::{BTreeMap, HashMap},
    future::{ready, Ready},
    rc::Rc,
};

#[derive(Debug, Clone)]
pub struct AuthData {
    pub user_id: i32,
}

pub fn verify_jwt(token: &str) -> Result<AuthData> {
    // TODO: move this to startup so it does not throw an error at runtime
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET NOT SET!");
    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes())?;
    let claims: BTreeMap<String, String> = token.verify_with_key(&key)?;
    Ok(AuthData {
        user_id: claims["user_id"].parse()?,
    })
}

fn get_token(req: &ServiceRequest) -> anyhow::Result<String> {
    let token_header = req
        .headers()
        .get("Authorization")
        .ok_or(anyhow!("No Authorization Token found"))?;
    let token = token_header.to_str()?;
    Ok(token.replace("Bearer ", "").trim().into())
}

pub fn get_header_http(req: &HttpRequest, header_name: &str) -> anyhow::Result<String> {
    let header = req
        .headers()
        .get(header_name)
        .ok_or(anyhow!("No {} header found!", header_name))?;
    Ok(header.to_str()?.to_string())
}

pub async fn get_token_query(query_string: &str) -> anyhow::Result<String> {
    let map = Query::<HashMap<String, String>>::from_query(query_string)?;
    let token = map.get("token").ok_or(anyhow!("No query token found"))?;
    Ok(token.into())
}

async fn authenticate(req: &ServiceRequest) -> Result<AuthData, Error> {
    let token = match get_token(req) {
        Ok(token) => token,
        Err(_e) => match get_token_query(req.query_string()).await {
            Ok(token) => token,
            Err(_e) => return Err(ErrorUnauthorized("Token not found")),
        },
    };

    match verify_jwt(&token) {
        Ok(claims) => Ok(AuthData {
            user_id: claims.user_id,
        }),
        Err(e) => {
            println!("{}", e);
            Err(ErrorUnauthorized(e))
        }
    }
}

pub struct AuthenticationMiddleware<S> {
    auth_data: Rc<AuthData>,
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Error>>;

    actix_service::forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        // Clone the Rc pointers so we can move them into the async block.
        let srv = self.service.clone();

        async move {
            // See if we can match it to a user.
            let auth = authenticate(&req).await?;
            req.headers_mut().insert(
                HeaderName::from_static("user_id"),
                HeaderValue::from_str(&auth.user_id.to_string())?,
            );
            // If we found a user, add it to the request extensions
            // for later retrieval.
            req.extensions_mut().insert::<AuthData>(auth);
            let res = srv.call(req).await?;

            Ok(res)
        }
        .boxed_local()
    }
}

pub struct AuthenticationMiddlewareFactory {
    auth_data: Rc<AuthData>,
}

impl AuthenticationMiddlewareFactory {
    pub fn new(auth_data: AuthData) -> Self {
        AuthenticationMiddlewareFactory {
            auth_data: Rc::new(auth_data),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthenticationMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthenticationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware {
            auth_data: self.auth_data.clone(),
            service: Rc::new(service),
        }))
    }
}
pub struct Auth(AuthData);

impl FromRequest for Auth {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let value = req.extensions().get::<AuthData>().cloned();
        let result = match value {
            Some(v) => Ok(Auth(v)),
            None => Err(ErrorUnauthorized("no auth data found")),
        };
        ready(result)
    }
}

impl std::ops::Deref for Auth {
    type Target = AuthData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
