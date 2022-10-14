use crate::CONTEXT;
use super::{domain::*, error::PostError};
use ic_cdk_macros::{update, query};

#[query]
fn query_posts(query: PostPageQuery) -> Result<PostPage, PostError> {
    CONTEXT.with(|c| {
        let ctx = c.borrow();
        Ok(ctx.post_service.getposts( &query))
    })
}

#[update]
fn create_post(cmd: PostCreateCommand) -> Result<u64, PostError> {
    CONTEXT.with(|c| {
        let mut ctx = c.borrow_mut();
        let id = ctx.id;
        let caller = ctx.env.caller();
        let now = ctx.env.now();
        let is_invited = true;
        let user_other_id = cmd.user_other_id.clone();
        let user_other_name = cmd.user_other_name.clone();
        let text = cmd.text.clone();
        let post = cmd.build_profile(
            id,
            now,
            caller.to_string(),
            is_invited,
            user_other_id,
            user_other_name,
            text
        );
        match ctx.post_service.create_post(post) {
            Some(_) => {
                ctx.id += 1;    // id addOne
                Ok(id)
            },
            None => Err(PostError::PostAlreadyExists),
        }
    })
}

// #[update(name = "linkByInvitationCode")]
#[update(name = "getInvitationCode")]
fn get_invitte_code(cmd: PostCreateCommand) -> Result<String, PostError> {
    CONTEXT.with(|c| {
        let mut ctx = c.borrow_mut();
        let id = ctx.id;
        let caller = ctx.env.caller();
        let now = ctx.env.now();
        let is_invited = false;
        let user_other_id = cmd.user_other_id.clone();
        let user_other_name = cmd.user_other_name.clone();
        let text = cmd.text.clone();

        let post = cmd.build_profile(
            id,
            now,
            caller.to_string(),
            is_invited,
            user_other_id,
            user_other_name,
            text
        );
        match ctx.post_service.get_invitation_code(post) {
            Some(code) => {
                ctx.id += 1;    // id addOne
                Ok(code)
            },
            None => Err(PostError::InviteFailError),
        }
    })
}

#[update(name = "linkByInvitationCode")]
fn link_by_invitation_code(invite_code:String) -> Result<Post, PostError> {
    CONTEXT.with(|c| {
        let mut ctx = c.borrow_mut();
        match ctx.post_service.link_by_invitation_code(invite_code) {
            Ok(post) => {
                Ok(post)
            },
            e=>e,
        }
    })
}