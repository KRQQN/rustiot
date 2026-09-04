use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};

use shared::dto::CurrentUser;

pub async fn protected_pages(req: Request, next: Next) -> Response {
    let path = req.uri().path();

    let protected = matches!(path, "/dashboard");

    if protected && req.extensions().get::<CurrentUser>().is_none() {
        return Redirect::to("/login").into_response();
    }

    next.run(req).await
}
