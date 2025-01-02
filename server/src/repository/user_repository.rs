use crate::{
    db::PgPool,
    models::user::{CreateUserRequest, LoginUserRequest, User},
    util::hash_password,
};
use actix_web::web::{self};
use rand::distributions::{Alphanumeric, DistString};
use uuid::Uuid;

pub struct UserRepository;

impl UserRepository {
    pub async fn login(
        pool: &PgPool,
        login_user: web::Json<LoginUserRequest>,
    ) -> Result<User, String> {
        let user = Self::get_user_by_email(pool, &login_user.email).await;

        let user = user.ok_or_else(|| "User email or password is invalid".to_string());

        let user = user?;

        let password_hash = hash_password(&login_user.password, &user.password_salt);

        if password_hash != user.password_hash {
            return Err("User email or password is invalid".to_string());
        }

        Ok(user)
    }

    pub async fn register(
        pool: &PgPool,
        new_user: web::Json<CreateUserRequest>,
    ) -> Result<User, String> {
        let user = Self::get_user_by_email(pool, &new_user.email).await;

        if user.is_some() {
            return Err("Email is already in use".to_string());
        }

        let password_salt = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

        let password_hash = hash_password(&new_user.password, &password_salt);

        let user = sqlx::query_as::<_, User>(
            r"
        INSERT INTO users (username, email, password_salt, password_hash) VALUES ($1, $2, $3, $4) returning *;
        ",
        )
        .bind(new_user.username.to_string())
        .bind(new_user.email.to_lowercase())
        .bind(password_salt)
        .bind(password_hash)
        .fetch_one(pool)
        .await;

        user.or(Err(
            "Something went wrong. Please try again later".to_string()
        ))
    }

    pub async fn get_user_by_email(pool: &PgPool, user_email: &str) -> Option<User> {
        let user = sqlx::query_as::<_, User>(
            r"
        SELECT * FROM users WHERE email = $1;
        ",
        )
        .bind(user_email.to_lowercase())
        .fetch_one(pool)
        .await;

        match user {
            Ok(user) => Some(user),
            Err(_) => None,
        }
    }

    pub async fn get_user_by_uuid(pool: &PgPool, user_id: &Uuid) -> Option<User> {
        let user = sqlx::query_as::<_, User>(
            r"
        SELECT * FROM users WHERE id = $1;
        ",
        )
        .bind(user_id)
        .fetch_one(pool)
        .await;

        match user {
            Ok(user) => Some(user),
            Err(_) => None,
        }
    }
}
