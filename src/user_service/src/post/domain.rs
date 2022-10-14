use std::{
    collections::VecDeque,
    string::ParseError,
    str::FromStr
};

use candid::{CandidType, Deserialize, Principal};
pub type PostId = u64;

#[derive(Debug, Clone, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Post {
    pub id: u64,
    pub timestamp: u64,
    pub user_self_id: String,
    pub user_other_id: String,
    pub user_other_name: String,
    pub text: String,
    pub is_invited: bool,
}

impl Post {
    pub fn new(id: u64, timestamp: u64, user_self_id: String, user_other_id: String,user_other_name:String,text:String,is_invited:bool) -> Self {
        Self {
            id,
            timestamp,
            user_self_id,
            user_other_id,
            user_other_name,
            text,
            is_invited
        }
    }
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct PostCreateCommand {
    pub user_other_id: String,
    pub user_other_name: String,
    pub text: String,
}

impl PostCreateCommand {
    pub fn build_profile(self, id: u64, timestamp: u64, user_self_id: String,is_invited:bool, user_other_id: String,user_other_name:String,text:String) -> Post {
        Post::new(id, timestamp,user_self_id,user_other_id,user_other_name,text,is_invited)
    }

}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct PostPage {
    pub data: Vec<Post>,
    pub page_size: usize,
    pub page_num: usize,
    pub total_count: usize,
}

#[derive(Debug, Clone, CandidType, Deserialize)]
pub struct PostPageQuery {
    pub page_size: usize,
    pub page_num: usize,
    pub user_id: String,
    pub text: String,
}