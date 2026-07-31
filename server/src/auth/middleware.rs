use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};

pub async fn auth_middleware(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    println!("Middleware triggered!");
    Ok(next.run(req).await)
}
