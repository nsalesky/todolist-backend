use bcrypt::{DEFAULT_COST, hash, verify};
use diesel::pg::Pg;
use diesel::prelude::*;
use rocket::response::Debug;
use uuid::Uuid;

use crate::auth::UserToken;
use crate::database::PostgresDbConn;
use crate::schema::users;
use crate::schema::users::dsl::*;

/// An object representing a full row in the users table.
#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub preferred_name: String,
    pub password_hash: String,
}

/// An object representing the information necessary to construct and insert a new user into the
/// users table. Most notably, the password has not been hashed yet.
#[derive(Serialize, Deserialize)]
pub struct UserDTO {
    pub username: String,
    pub email: String,
    pub preferred_name: String,
    pub password: String,
}

/// An object representing a new row that can be inserted into the users table.
#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub preferred_name: String,
    pub password_hash: String,
}

/// A JSON object containing the information the user passes to log in to their account
#[derive(Serialize, Deserialize)]
pub struct LoginDTO {
    pub username_or_email: String,
    pub password: String,
}

/// The user's information that will be encoded into a UserToken and returned to them
/// on successfully logging in
// #[derive(Insertable)]
// #[table_name = "users"]
pub struct LoginInfoDTO {
    pub id: i32,
    pub username: String,
}

impl User {
    /// Hash the password for the given user and attempt to insert them to the users table.
    pub fn signup(user: UserDTO, conn: &PgConnection) -> bool {
        let new_hash = hash(&user.password, DEFAULT_COST).unwrap();

        let user = NewUser {
            username: user.username,
            email: user.email,
            preferred_name: user.preferred_name,
            password_hash: new_hash,
        };

        diesel::insert_into(users)
            .values(&user)
            .execute(conn)
            .is_ok()
    }

    /// Attempts to login the user with the specified login information.
    /// If successful, returns a LoginInfoDTO containing the information that will be encoded
    /// into their login token.
    /// If unsuccessful, returns None.
    pub fn login(login: LoginDTO, conn: &PgConnection) -> Option<LoginInfoDTO> {
        let unverified_user = users
            .filter(username.eq(&login.username_or_email))
            .or_filter(email.eq(&login.username_or_email))
            .get_result::<User>(conn)
            .unwrap();

        if !unverified_user.password_hash.is_empty()
            && verify(&login.password, &unverified_user.password_hash).unwrap() {
            Some(LoginInfoDTO {
                id: unverified_user.id,
                username: unverified_user.username,
            })
        } else {
            None
        }
    }

    /// Checks whether the given token is valid, ie whether it corresponds to a real user in
    /// the users table.
    pub async fn is_valid_login_token(user_token: &UserToken, conn: &PgConnection) -> bool {
        users
            .filter(id.eq(&user_token.id))
            .filter(username.eq(&user_token.username))
            .get_result::<User>(conn)
            .is_ok()
    }

    /// Tries to find the user with the given unique username in the users table.
    pub fn find_user_by_username(un: &str, conn: &PgConnection) -> Option<User> {
        let possible_user = users.filter(username.eq(un)).get_result::<User>(conn);
        if let Ok(user) = possible_user {
            Some(user)
        } else {
            None
        }
    }

    /// Tries to find the user with the given id in the users table.
    pub fn find_user_by_id(user_id: i32, conn: &PgConnection) -> Option<User> {
        let possible_user = users.filter(id.eq(user_id)).get_result::<User>(conn);
        if let Ok(user) = possible_user {
            Some(user)
        } else {
            None
        }
    }
}
