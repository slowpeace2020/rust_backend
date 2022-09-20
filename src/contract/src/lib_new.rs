
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::str;
use std::ops::Add;

use ic_cdk::{
    export::{
        candid::{CandidType, Deserialize},
    },
};
use ic_cdk::*;
use ic_cdk_macros::*;
use ic_cdk::api::time;
use serde_json::{Value};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::collections::HashMap;
use std::{
    collections::BTreeMap,
    cmp::Ordering,
    str::FromStr,
};
use candid::Principal;

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

#[derive(Debug, Clone, CandidType, Deserialize)]
pub enum InviteError {
    InvitationCodeNotFound,
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
    pub user_id: Option<String>,
    pub text: Option<String>,
}


type Contract = Vec<Post>;

type LatestPostId = i128;

type InvitationPost = BTreeMap<String, Post>;

type PostsMap = BTreeMap<LatestPostId, Post>;

#[update(name = "getInvitationCode")]
fn get_invitation_code(text: String) -> String {
    let principal_id = ic_cdk::caller();

    let s = get_invite_code_hash();

    let invitation_post_store = storage::get_mut::<InvitationPost>();


    // contract content
    let latest_post_id = storage::get_mut::<LatestPostId>();
    *latest_post_id = latest_post_id.add(1);

    let post = Post {
        id: *latest_post_id,
        timestamp: time() as i128,
        timesdelta: 0,
        user_self_id: principal_id.to_string(),
        user_other_id: String::from(s.clone()),
        text,
    };
    crate::println!("post: {:?}",post);
    invitation_post_store.insert(s.clone().parse().unwrap(),post);
    return String::from(&s)
}

fn get_invite_code_hash() -> String{
    let latest_post_id = storage::get_mut::<LatestPostId>();
    let mut s1 = latest_post_id.clone().to_string();
    let principal_id = ic_cdk::caller();
    let s2 = principal_id.to_string();
    s1 += &s2;
    let s: String = s1.clone();
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    // let mut h = hasher.finish()%(time() as u64);
    // h = h%((time()/1000000) as u64);
    let h = hasher.finish()%(time() as u64);

    let code = base_n(h,64);

    //check if the invitation code exists
    let invitation_post_store = storage::get_mut::<InvitationPost>();
    let mut post = invitation_post_store.get(&*code.clone());
    match post.as_mut() {
        Some(_post) =>  {
            return get_invite_code_hash();
        },
        None => {
            return code;
        },
    }
}

/// 10 进制转为 11 - 64 进制 36 进制前是小写
fn base_n(num: u64, n: i32) -> String {
    let num_rep: HashMap<i32, char> = HashMap::from([
        (10, 'a'), (11, 'b'), (12, 'c'), (13, 'd'), (14, 'e'),
        (15, 'f'), (16, 'g'), (17, 'h'), (18, 'i'), (19, 'j'),
        (20, 'k'), (21, 'l'), (22, 'm'), (23, 'n'), (24, 'o'),
        (25, 'p'), (26, 'q'), (27, 'r'), (28, 's'), (29, 't'),
        (30, 'u'), (31, 'v'), (32, 'w'), (33, 'x'), (34, 'y'),
        (35, 'z'),
        (36, 'A'), (37, 'B'), (38, 'C'), (39, 'D'), (40, 'E'),
        (41, 'F'), (42, 'G'), (43, 'H'), (44, 'I'), (45, 'J'),
        (46, 'K'), (47, 'L'), (48, 'M'), (49, 'N'), (50, 'O'),
        (51, 'P'), (52, 'Q'), (53, 'R'), (54, 'S'), (55, 'T'),
        (56, 'U'), (57, 'V'), (58, 'W'), (59, 'X'), (60, 'Y'),
        (61, 'Z'), (62, '@'), (63, '*'),
    ]);

    let mut new_num_string = String::from("");
    let mut current: u64 = num;

    while current != 0 {
        let remainder = (current % (n as u64)) as i32;
        let mut remainder_string: String;

        if remainder > 9 && remainder < 64 {
            remainder_string = format!("{}", num_rep.get(&remainder).unwrap());
        } else {
            remainder_string = format!("{}", remainder);
        }

        new_num_string = format!("{}{}", remainder_string, new_num_string);
        current = current / (n as u64);
    }

    if new_num_string.len()>8 {
        new_num_string = new_num_string.split_off(new_num_string.len()-8);
    }else{
        return get_invite_code_hash();
    }

    new_num_string
}

fn get_invite_code() -> String{
    let rand_string:Vec<char> = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .collect();

    let s: String = rand_string.iter().collect();

    let invitation_store = storage::get_mut::<InvitationPost>();

    let ans = String::from(s);


    let mut post = invitation_store.get(&*ans.clone());
    match post.as_mut() {
        Some(_post) =>  {
            return get_invite_code();
        },
        None => {
            return ans;
        },
    }

}

#[update(name = "linkByInvitationCode")]
fn link_by_invitation_code(invitation_code:String) -> Result<Post, InviteError> {
        let invitation_post_store = storage::get_mut::<InvitationPost>();
        //replace user B's principal_id into post
        if invitation_post_store.contains_key(&invitation_code){
            let mut post = invitation_post_store.get(&invitation_code).unwrap().clone();
            let principal_id = ic_cdk::caller();
            post.user_other_id = principal_id.to_string();
            crate::println!("==================================");
            crate::println!("post: {:?}",post);
            crate::println!("==================================");
            let wall = storage::get_mut::<Contract>();
            wall.push(post);

            let mut post_contract = invitation_post_store.get(&invitation_code);
            match post_contract.as_mut() {
                Some(contract) =>  {
                    crate::println!("contract: {:?}",contract);
                    _remove_code(invitation_code.clone());
                    //todo return principal
                    // return String::from("{\"info\":\"link by invitation code succeeded!\"}");
                    return Ok(contract.clone());
                }
                None => {
                },
            }

        }

        Err(InviteError::InvitationCodeNotFound)
}

fn _remove_code(invitation_code:String){
    let invitation_post_store = storage::get_mut::<InvitationPost>();
    invitation_post_store.remove(&invitation_code);
}

// 分页查询 post 内容，没有 comment
pub fn posts(q: &PostPageQuery) -> PostPage {

    let pages = posts_query(q);

    PostPage {
        page_size: pages.page_size,
        page_num: pages.page_num,
        total_count: pages.total_count,
        data: pages.data.into_iter().map(|p| p.into()).collect()
    }
}

// 分页查询 post and comment 内容
pub fn posts_query(q: &PostPageQuery)-> PostPage {

    let page_size= q.page_size;
    let page_num = q.page_num;
    let filter = |p: && Post| (q.user_id.is_empty()||(p.user_self_id==q.user_id)||(p.user_other_id==q.user_id)) &&
        (q.querystring.is_empty() || p.text.contains(&q.querystring));
    let ps = storage::get_mut::<PostsMap>();

    let compare = |p1:&PostPage, p2: &PostPage| p2.created_at.cmp(&p1.created_at);
    paging(ps, page_size, page_num, filter, compare)
}


fn paging(ps: &BTreeMap<i128, Post>, page_size: usize, page_num: usize,
          ff: impl Fn(&&Post) -> bool, compare: impl Fn(&Post, &Post) -> Ordering)
          -> PostPage {
    let mut ps: Vec<Post> = ps
        .values()
        .filter(ff)
        .cloned()
        .collect();

    ps.sort_by(compare);

    let total_count = ps.len();
    let data = ps.iter().skip(page_num * page_size).take(page_size).cloned().collect();
    PostPage { page_num, page_size, total_count, data }
}

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