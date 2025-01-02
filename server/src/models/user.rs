use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize, Serializer};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct User {
    #[serde(skip_serializing)]
    pub id: Uuid,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_salt: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    #[serde(rename = "createdAt")]
    #[serde(serialize_with = "ndt_to_string")]
    pub created_at: NaiveDateTime,
    #[serde(rename = "updatedAt")]
    #[serde(serialize_with = "ndt_to_string")]
    pub updated_at: NaiveDateTime,
}

fn ndt_to_string<S>(ndt: &NaiveDateTime, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&ndt.format("%Y-%m-%d %H:%M:%S").to_string())
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 4))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
    #[serde(alias = "passwordConfirm")]
    #[validate(must_match(other = "password"))]
    pub password_confirm: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserDto {
    pub username: String,
    pub email: String,
}
