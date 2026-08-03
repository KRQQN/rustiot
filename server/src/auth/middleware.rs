use app::AppState;
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use shared::{
    dto::CurrentUser,
    jwt::{self},
};
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

    if let Some(claims) = claims {
        let current_user = CurrentUser {
            user_id: claims.sub,
            email: claims.email,
        };
        req.extensions_mut().insert(current_user);
    }

    next.run(req).await
}
