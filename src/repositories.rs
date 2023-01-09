mod user;

pub use user::*;

use std::marker::{Send, Sync};

pub trait Repositories<Conn, Tran>: Send + Sync + 'static {
    type UserRepo: UserRepository<Conn, Tran>;

    fn user(&self) -> &Self::UserRepo;
}
