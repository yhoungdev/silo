pub mod file_action;

use axum:: { Router , routing::get, routing::post };

pub fn routes () -> Router {
    Router::new()
        .route("/upload" , get(file_action::upload_file))
        .route("/preview" , get(file_action::preview_uploaded_files))
}