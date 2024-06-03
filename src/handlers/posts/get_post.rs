use axum::extract::{Path, State};
use axum::Json;
use uuid::Uuid;

use crate::domain::models::post::PostError;
use crate::handlers::posts::PostResponse;
use crate::infra::repositories::post_repository;
use crate::AppState;

pub async fn get_post(
    State(state): State<AppState>,
    Path(post_id): Path<Uuid>,
) -> Result<Json<PostResponse>, PostError> {
    let post = post_repository::get(&state.pool, post_id).await?;

    Ok(Json(post.into()))
}
