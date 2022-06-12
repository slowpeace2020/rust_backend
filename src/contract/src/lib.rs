use std::ops::Add;

use ic_cdk::export::{
    candid::{CandidType, Deserialize},
    Principal,
};
use ic_cdk::*;
use ic_cdk_macros::*;
use ic_cdk::api::time;
use serde_json::{Value};


const PAGESIZE: usize = 25;

#[derive(Clone, Debug, CandidType, Deserialize)]
struct PostPreUpgrade {
    pub user_self: Principal,
    pub user_other: Principal,
    pub text: String,
}
type ContractPreUpgrade = Vec<PostPreUpgrade>;

#[derive(Clone, Debug, CandidType, Deserialize)]
struct Post {
    pub id: i128,
    pub timestamp: i128,
    pub timesdelta: i128,
    pub user_self_id: String,
    pub user_other_id: String,
    pub text: String,
}
type Contract = Vec<Post>;

type LatestPostId = i128;

fn paginate(posts: Vec<&Post>, page: usize) -> Vec<&Post> {
    let start_index = posts.len() - ((page - 1) * PAGESIZE) - 1;
    let mut paginated_posts = Vec::new();
    let mut n: usize = 0;
    while n < PAGESIZE && n <= start_index {
        // let mut post = Post {
        //     id: posts[start_index - n].id,
        //     timestamp: posts[start_index - n].timestamp,
        //     timesdelta: time() as i128 - posts[start_index - n].timestamp ,
        //     user_self_id: posts[start_index - n].user_self_id,
        //     user_other_id: posts[start_index - n].user_other_id,
        //     text: posts[start_index - n].text,
        // };
        // posts[start_index - n].timesdelta = time() as i128 - posts[start_index - n].timestamp;
        paginated_posts.push(posts[start_index - n]);
        n += 1;
    }
    paginated_posts
}


#[query]
fn message(filter_json: String) -> Vec<&'static Post> {
    crate::println!("filter_json: {}",filter_json);
    let wall_posts = storage::get::<Contract>();

    let filter: Value = serde_json::from_str(&filter_json).unwrap();
    // PASS 1, filter on user_id
    let pass1 = match filter["user_self_id"].is_string() {
        true => {
            wall_posts
            .iter()
            .filter_map(|p| match p.user_self_id == filter["user_self_id"] || p.user_other_id == filter["user_other_id"] {
                true => Some(p),
                false => None
            })
            .collect::<Vec<&Post>>()
        },
        false => wall_posts.iter().map(|p| p).collect::<Vec<&Post>>()
    };

    // PASS 2, pagination
    match filter["page"].is_number() {
        true => {
            let page = filter["page"].as_i64().unwrap() as usize;
            paginate(pass1, page)
        },
        false => pass1.iter().map(|&p| p).collect()
    }


    //===============for test=====================
    //test data
    // let mut post = Post {
    //     id: 1,
    //     timestamp: time() as i128,,
    //     user_self_id: "for test1".to_string(),
    //     user_other_id: "for test2".to_string(),
    //     text: "test message".to_string(),
    // };

    // let mut test_data: Vec<&Post> = Vec::new();
    // test_data.push(&post);
    // test_data
    //===============for test=====================
}

#[update]
fn write(text: String,other_principal_id: String)  {
    let principal_id = ic_cdk::caller().to_string();
    let latest_post_id = storage::get_mut::<LatestPostId>();
    *latest_post_id = latest_post_id.add(1);

    let post = Post {
        id: *latest_post_id,
        timestamp: time() as i128,
        timesdelta: 0,
        user_self_id: principal_id,
        user_other_id: other_principal_id,
        text,
    };

    let wall = storage::get_mut::<Contract>();
    wall.push(post);
}

#[pre_upgrade]
fn pre_upgrade() {
    let wall = storage::get::<ContractPreUpgrade>();
    storage::stable_save((wall,)).unwrap();
    return;
}

#[post_upgrade]
fn post_upgrade() {
    let wall = storage::get_mut::<Contract>();
    let latest_post_id = storage::get_mut::<LatestPostId>();

    let res:Result<(Vec<PostPreUpgrade>,), String> = storage::stable_restore();
    match res {
        Ok((old_posts,)) => {
            for old_post in old_posts {
                ic_cdk::println!("Upgrading post");
                *latest_post_id = latest_post_id.add(1);
                wall.push(Post {
                    id: *latest_post_id,
                    timestamp: time() as i128,
                    timesdelta: 0,
                    user_self_id: old_post.user_self.to_string(),
                    user_other_id: old_post.user_other.to_string(),
                    text: old_post.text
                });
            }
            return;
        }
        Err(_) => return
    }
}