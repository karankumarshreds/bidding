use crate::handlers::auth::{
    validate_token,
    JWT_KEY,
};
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

    println!("validating token with \n JWT KET: {} & AUTH_HEADER: {}" , JWT_KEY, auth_header);
    let claims = validate_token(auth_header, JWT_KEY)
        .map_err(|err|{
            println!("what the fucccc {:#?}", err);
            return StatusCode::UNAUTHORIZED;
        })?;
    println!("verified: claims {:?}", claims);
    req.extensions_mut().insert(claims);
    Ok(next.run(req).await)
}
