use actix_web::web;
mod login;
mod logout;

// if we use super in a mod.rs, we import files in the parent directory mod.rs file.
use super::path::Path;

// instead of passing actix_web::App, we pass ServiceConfig struct to define routes.
pub fn auth_factory(app: &mut web::ServiceConfig) {
    let base_path: Path = Path {
        prefix: String::from("/auth"),
    };

    app.route(
        &base_path.define(String::from("/login")),
        web::get().to(login::login),
    )
    .route(
        &base_path.define(String::from("/logout")),
        web::get().to(login::login),
    )
}
