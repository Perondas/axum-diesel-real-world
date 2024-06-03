use axum::extract::{Query, State};
use axum::Json;

use crate::domain::models::post::PostError;
use crate::handlers::posts::PostResponse;
use crate::infra::repositories::post_repository::{get_all, PostsFilter};
use crate::AppState;

pub async fn list_posts(
    State(state): State<AppState>,
    Query(params): Query<PostsFilter>,
) -> Result<Json<Vec<PostResponse>>, PostError> {
    let posts = get_all(&state.pool, params).await?;

    Ok(Json(posts.into_iter().map(|post| post.into()).collect()))
}
