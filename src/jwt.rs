use chrono::Utc;
use diesel::PgConnection;
use diesel::prelude::*;
use jsonwebtoken::{Header, Validation};
use jsonwebtoken::{DecodingKey, EncodingKey};
use jsonwebtoken::errors::Result;
use jsonwebtoken::TokenData;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome, Request};
use rocket::response::content::Json;
use rocket::response::status;

use crate::constants;
use crate::database::PostgresDbConn;
use crate::models::response::Response;
use crate::models::user::{LoginInfoDTO, User};
use crate::schema::users::dsl::*;

static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // Number of seconds in a week

#[derive(Debug, Serialize, Deserialize)]
pub struct UserToken {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // data
    pub user: String,
    pub login_session: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserToken {
    type Error = status::Custom<Json<Response>>;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let db = request.rocket().state::<PostgresDbConn>().unwrap();

        if let Some(authen_header) = request.headers().get_one("Authorization") {
            let authen_str = authen_header.to_string();

            if authen_str.starts_with("Bearer") {
                let token = authen_str[6..authen_str.len()].trim();
                if let Ok(token_data) = decode_token(token.to_string()) {
                    if verify_token(&token_data, db) {
                        return Outcome::Success(token_data.claims);
                    }
                }
            }
        }

        Outcome::Failure((
            Status::BadRequest,
            status::Custom(
                Status::Unauthorized,
                Json(Response {
                    message: String::from(constants::MESSAGE_INVALID_TOKEN),
                    data: serde_json::to_value("").unwrap(),
                }),
            ),
        ))
    }
}

pub fn generate_token(login: LoginInfoDTO) -> String {
    let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
    let payload = UserToken {
        iat: now,
        exp: now + ONE_WEEK,
        user: login.username,
        login_session: login.login_session,
    };

    jsonwebtoken::encode(&Header::default(), &payload, &EncodingKey::from_secret(include_bytes!("secret.key"))).unwrap()
}

fn decode_token(token: String) -> Result<TokenData<UserToken>> {
    jsonwebtoken::decode::<UserToken>(&token, &DecodingKey::from_secret(include_bytes!("secret.key")), &Validation::default())
}

// async fn verify_token(token_data: &TokenData<UserToken>, db: &PostgresDbConn) -> bool {
//     db.run(move |conn| {
//         User::is_valid_login_session(&token_data.claims, conn)
//     }).await
// }

fn verify_token(token_data: &TokenData<UserToken>, conn: &PostgresDbConn) -> bool {
    User::is_valid_login_session(&token_data.claims, &conn.0)
}