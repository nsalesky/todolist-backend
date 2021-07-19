use std::sync::{Arc, RwLock};
use crypto::digest::Digest;
use crypto::sha3::Sha3;
use rocket::{Request, request, State};
use rocket::request::FromRequest;
use serde::Deserialize;
use rocket::tokio::sync::Mutex;

#[derive(Deserialize)]
pub struct LoggedInUser {
    username: String,
    password: String,
}

#[derive(Deserialize, Debug)]
pub struct CreateUser {
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

/// Hashes the given password using SHA-3.
pub fn hash_password(password: &String) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input_str(password);
    hasher.result_str()
}