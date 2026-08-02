use app::AppState;
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use shared::jwt::{self, Claims};
use tower_cookies::{Cookie, Cookies};

const SESSION_COOKIE: &str = "session";

pub async fn auth_middleware(
    State(_app): State<AppState>,
    cookies: Cookies,
    mut req: Request,
    next: Next,
) -> Response {
    let token = cookies.get(SESSION_COOKIE).map(|c| c.value().to_string());

    let claims = match token.as_deref() {
        Some(t) => match jwt::verify(t) {
            Ok(claims) => Some(claims),
            Err(_) => {
                cookies.remove(Cookie::from(SESSION_COOKIE));
                None
            }
        },
        None => None,
    };
    dbg!("token!: {}", &token);

    // todo: Change to shared type
    if let Some(claims) = claims {
        dbg!("claims!: {}", &claims);
        req.extensions_mut().insert(claims);
    }

    next.run(req).await
}
