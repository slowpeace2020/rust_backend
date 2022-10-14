use candid::Principal;
use super::{domain::*, error::PostError};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use ic_cdk::api::time;
use std::{
    collections::{HashMap, BTreeMap},
    cmp::Ordering,
    str::FromStr,
};

pub type PostId = u64;

#[derive(Debug, Default)]
pub struct PostService {
    pub posts: BTreeMap<PostId, Post>,
    pub invitations: BTreeMap<String, PostId>
}


impl PostService {
    // 分页查询 post 内容，没有 comment
    pub fn getposts(&self, q: &PostPageQuery) -> PostPage {

        // let page_size= q.page_size;
        // let page_num = q.page_num;
        // let filter = |p: &&PostProfile| p.author == caller &&
        //     (q.querystring.is_empty() || (p.title.contains(&q.querystring) || p.text.text.contains(&q.querystring)));
        // let ps = &self.posts;

        // let compare = |p1:&PostProfile, p2: &PostProfile| p2.created_at.cmp(&p1.created_at);
        // let profiles = paging(ps, page_size, page_num, filter, compare);
        let pages = self.posts_query(q);

        PostPage {
            page_size: pages.page_size,
            page_num: pages.page_num,
            total_count: pages.total_count,
            data: pages.data.into_iter().map(|p| p.into()).collect()
        }
    }

    // 分页查询 post and comment 内容
    pub fn posts_query(&self, q: &PostPageQuery)-> PostPage {

        let page_size= q.page_size;
        let page_num = q.page_num;
        let filter = |p: &&Post| {

            let mtach_self_id = q.user_id.is_empty() || p.user_self_id==q.user_id;
            let match_other_id = q.user_id.is_empty() || p.user_other_id==q.user_id;

            match_other_id&&mtach_self_id && (q.text.is_empty() || p.text.contains(&q.text))
        };

        let ps = &self.posts;

        let compare = |p1:&Post, p2: &Post| p2.id.cmp(&p1.id);
        paging(ps, page_size, page_num, filter, compare)
    }




    pub fn create_post(&mut self, post: Post) -> Option<u64> {
        let id = post.id;
        match self.posts.get(&id) {
            Some(_) => None,
            None => {
                self.posts.insert(
                    id,
                    post,
                );
                Some(id)
            }
        }
    }

   pub fn get_invitation_code(&mut self, mut post:Post) -> Option<String> {
       let invitation_code = String::from(&self.get_invite_code_hash(post.id));
       let mut invitation_post_store = &self.invitations;
       if invitation_post_store.contains_key(&invitation_code){
           return self.get_invitation_code(post);
       }
       post.user_other_id = invitation_code.clone();

       let post_id = post.id;
       match self.create_post(post.clone()){
           Some(_)=>{
               &self.invitations.insert(invitation_code.clone(),post_id);
               &self.posts.insert(post_id,post.clone());
               Some(invitation_code.clone())
           },
           None => {
               None
           }
       }

    }

    fn get_invite_code_hash(&mut self, post_id:u64) -> String {
        let mut s1 = post_id.clone().to_string();
        let principal_id = ic_cdk::caller();
        let s2 = principal_id.to_string();
        s1 += &s2;
        let s: String = s1.clone();
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        // let mut h = hasher.finish()%(time() as u64);
        // h = h%((time()/1000000) as u64);
        let h = hasher.finish()%(time() as u64);

        let code = &self.base_n(h,64,post_id);

        return code.clone();
    }



  pub  fn link_by_invitation_code(&mut self, invitation_code:String) -> Result<Post, PostError> {
        let mut invitation_post_store = &self.invitations;
        let mut posts_map_store = &self.posts.clone();

        //replace user B's principal_id into post
        if invitation_post_store.contains_key(&invitation_code){
            let mut post_id = invitation_post_store.get(&invitation_code).unwrap().clone();

            let mut post_contract = posts_map_store.get(&post_id);
            match post_contract.as_mut() {
                Some(_) =>  {
                    _remove_code(&mut self.invitations, invitation_code.clone());
                    let mut post = posts_map_store.get(&post_id).unwrap().clone();
                    let principal_id = ic_cdk::caller();
                    post.user_other_id = principal_id.to_string();
                    post.is_invited = true;
                    self.posts.insert(post_id, post);

                    return Ok(posts_map_store.get(&post_id).unwrap().clone());
                }
                None => {

                },
            }

        }

      return Err(PostError::InviteCodeError);
    }

    /// 10 进制转为 11 - 64 进制 36 进制前是小写
    fn base_n(&mut self, num: u64, n: i32,post_id:u64) -> String {
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
            return String::from(&self.get_invite_code_hash(post_id));
        }

        new_num_string
    }

}

fn paging(ps: &BTreeMap<u64, Post>, page_size: usize, page_num: usize,
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

fn _remove_code(invitations: &mut BTreeMap<String, u64>, invitation_code:String){
    invitations.remove(&invitation_code);
}

