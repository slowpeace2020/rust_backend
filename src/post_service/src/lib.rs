extern crate ic_cdk;
extern crate candid;

use std::cell::RefCell;

use candid::Principal;


pub mod actor;
pub mod post;
pub mod env;
pub mod context;

use crate::context::DaoContext;

pub use post::*;

thread_local! {
    static CONTEXT: RefCell<DaoContext> = RefCell::default();
}