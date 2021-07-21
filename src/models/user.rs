use rocket::response::Debug;

use crate::schema::users::dsl::*;
use crate::schema::users;
use diesel::prelude::*;
use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;
use crate::auth::UserToken;
use crate::database::PostgresDbConn;
use diesel::pg::Pg;

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub preferred_name: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserDTO {
    pub username: String,
    pub email: String,
    pub preferred_name: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub preferred_name: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginDTO {
    pub username_or_email: String,
    pub password: String,
}

// #[derive(Insertable)]
// #[table_name = "users"]
pub struct LoginInfoDTO {
    pub id: i32,
    pub username: String,
}

impl User {
    /// Hash the password for the given user and attempt to add them to the database
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

    pub fn login(login: LoginDTO, conn: &PgConnection) -> Option<LoginInfoDTO> {
        let unverified_user = users
            .filter(username.eq(&login.username_or_email))
            .or_filter(email.eq(&login.username_or_email))
            .get_result::<User>(conn)
            .unwrap();

        if !unverified_user.password_hash.is_empty()
            && verify(&login.password, &unverified_user.password_hash).unwrap() {
            // let login_session_str = User::generate_login_session();
            // User::update_login_session_to_db(&unverified_user.username, &login_session_str, conn);

            Some(LoginInfoDTO {
                id: unverified_user.id,
                username: unverified_user.username,
            })
        } else {
            None
        }
    }

    pub async fn is_valid_login_token(user_token: &UserToken, conn: &PgConnection) -> bool {
        users
            .filter(id.eq(&user_token.id))
            .filter(username.eq(&user_token.username))
            .get_result::<User>(conn)
            .is_ok()
    }

    pub fn find_user_by_username(un: &str, conn: &PgConnection) -> Option<User> {
        let possible_user = users.filter(username.eq(un)).get_result::<User>(conn);
        if let Ok(user) = possible_user {
            Some(user)
        } else {
            None
        }
    }

    // pub fn generate_login_session() -> String {
    //     Uuid::new_v4().to_simple().to_string()
    // }
    //
    // pub fn update_login_session_to_db(un: &str, login_session_str: &str, conn: &PgConnection) -> bool {
    //     if let Some(user) = User::find_user_by_username(un, conn) {
    //         diesel::update(users.find(user.id))
    //             .set(login_session.eq(login_session_str.to_string()))
    //             .execute(conn)
    //             .is_ok()
    //     } else {
    //         false
    //     }
    // }
}
