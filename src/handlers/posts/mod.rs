use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use create_post::create_post;
pub use get_post::get_post;
pub use list_posts::list_posts;

use crate::{
    domain::models::post::PostModel,
    infra::repositories::post_repository::{NewPostDb, PostDb},
};

mod create_post;
mod get_post;
mod list_posts;

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    title: String,
    body: String,
}

impl From<CreatePostRequest> for NewPostDb {
    fn from(request: CreatePostRequest) -> Self {
        NewPostDb {
            title: request.title,
            body: request.body,
            published: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostResponse {
    id: Uuid,
    title: String,
    body: String,
    published: bool,
}

impl From<PostModel> for PostResponse {
    fn from(model: PostModel) -> Self {
        PostResponse {
            id: model.id,
            title: model.title,
            body: model.body,
            published: model.published,
        }
    }
}

impl From<PostDb> for PostModel {
    fn from(post: PostDb) -> Self {
        PostModel {
            id: post.id,
            title: post.title,
            body: post.body,
            published: post.published,
        }
    }
}
