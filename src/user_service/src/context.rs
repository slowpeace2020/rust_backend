
use candid::{CandidType, Deserialize, Principal};
use std::collections::BTreeMap;
use std::iter::FromIterator;

use crate::env::{Environment, CanisterEnvironment, EmptyEnvironment};
use crate::post::domain::*;

use crate::post::PostService;

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct DaoDataStorage {
    pub id: u64,
    pub posts: Vec<Post>,
}

impl From<DaoContext> for DaoDataStorage {
    fn from(context: DaoContext) -> Self {
        let id = context.id;
        let posts = Vec::from_iter(context.post_service.posts
            .iter()
            .map(|(_k, v)| v.clone()));
        Self {
            id,
            posts,
        }
    }
}

pub struct DaoContext {
    pub env: Box<dyn Environment>,
    pub id: u64,
    pub post_service: PostService,
}

impl Default for DaoContext {
    fn default() -> Self {
        Self {
            env: Box::new(EmptyEnvironment {}),
            id: 10001,
            post_service: PostService::default(),
        }
    }
}

impl From<DaoDataStorage> for DaoContext {
    fn from(payload: DaoDataStorage) -> Self {
        let posts: BTreeMap<PostId, Post> = payload
            .posts
            .into_iter()
            .map(|p| (p.id, p))
            .collect();

        let mut invitations: Vec<Post> = posts
            .values()
            .filter(|p: &&Post| { !p.is_invited })
            .cloned()
            .collect();
        let invitations: BTreeMap<String,PostId> = invitations
            .into_iter()
            .map(|p| (p.user_other_id,p.id))
            .collect();

        Self {
            env: Box::new(CanisterEnvironment {}),
            id: payload.id,
            post_service: PostService { posts, invitations},
        }
    }
}

#[cfg(test)]
mod tests {

}
