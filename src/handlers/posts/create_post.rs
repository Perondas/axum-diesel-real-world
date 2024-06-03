use axum::extract::State;
use axum::Json;

use crate::domain::models::post::PostError;
use crate::handlers::posts::{CreatePostRequest, PostResponse};
use crate::infra::repositories::post_repository;
use crate::utils::JsonExtractor;
use crate::AppState;

pub async fn create_post(
    State(state): State<AppState>,
    JsonExtractor(new_post): JsonExtractor<CreatePostRequest>,
) -> Result<Json<PostResponse>, PostError> {
    let created_post = post_repository::insert(&state.pool, new_post.into()).await?;

    Ok(Json(created_post.into()))
}
