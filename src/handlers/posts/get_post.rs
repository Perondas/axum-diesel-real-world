use axum::extract::State;
use axum::Json;
use uuid::Uuid;

use crate::domain::models::post::PostError;
use crate::handlers::posts::PostResponse;
use crate::infra::repositories::post_repository;
use crate::utils::PathExtractor;
use crate::AppState;

pub async fn get_post(
    State(state): State<AppState>,
    PathExtractor(post_id): PathExtractor<Uuid>,
) -> Result<Json<PostResponse>, PostError> {
    let post = post_repository::get(&state.pool, post_id).await?;

    Ok(Json(post.into()))
}
