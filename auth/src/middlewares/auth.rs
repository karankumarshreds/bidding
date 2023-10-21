use std::sync::Arc;
use crate::routes::validate_token;
use axum::{
    http::{Request, header},
    response::Response,
    http::StatusCode,
    middleware::Next,
};

pub async fn with_auth<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let auth_header = req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| {
            println!("header: {:#?}", h.to_str());
            return h.to_str().ok()
        })
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let claims = validate_token(auth_header, std::env::var("JWT_KEY").unwrap().as_ref())
        .map_err(|err|{
            println!("Error validating token: \n{:#?}", err);
            return StatusCode::UNAUTHORIZED;
        })?;

    println!("adding claims: {:#?}", claims);
    req.extensions_mut().insert(Arc::new(claims));
    println!("req extensions: {:#?}", req.extensions());
    Ok(next.run(req).await)
}

/*
    pub async fn guard<T>(Request<T>, ) {
    }
*/
