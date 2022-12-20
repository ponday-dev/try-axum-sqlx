mod user;

pub use user::*;

use std::marker::{Send, Sync};

pub trait Repositories: Send + Sync + 'static {
    type UserRepo: UserRepository;

    fn user(&self) -> &Self::UserRepo;
}
