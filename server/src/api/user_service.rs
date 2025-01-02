use actix_session::{Session, SessionGetError};
use actix_web::{
    web::{self},
    HttpResponse, Responder,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    db::PgPool,
    models::user::{CreateUserRequest, LoginUserRequest, UserDto},
    repository::user_repository::UserRepository,
};

pub fn user_service_config(config: &mut web::ServiceConfig) {
    let api_auth_login = web::resource("/login").route(web::post().to(login));
    let api_auth_register = web::resource("/register").route(web::post().to(register));
    let api_auth_logout = web::resource("/logout").route(web::post().to(logout));

    let api_get_user_by_id = web::resource("/user").route(web::get().to(get_user_by_id));

    config
        .service(
            web::scope("/auth")
                .service(api_auth_login)
                .service(api_auth_register)
                .service(api_auth_logout),
        )
        .service(api_get_user_by_id);
}

async fn get_user_by_id(pool: web::Data<PgPool>, session: Session) -> impl Responder {
    let session_value: Result<Option<Uuid>, SessionGetError> = session.get("user_id");

    let user_id = session_value.unwrap_or_default();

    if user_id.is_none() {
        return HttpResponse::Unauthorized().finish();
    }

    let user_id = user_id.unwrap();

    let user = UserRepository::get_user_by_uuid(&pool, &user_id).await;

    match user {
        Some(user) => {
            let user_dto = UserDto {
                username: user.username,
                email: user.email,
            };
            HttpResponse::Ok().json(user_dto)
        }
        None => HttpResponse::Unauthorized().finish(),
    }
}

async fn login(
    pool: web::Data<PgPool>,
    login_user: web::Json<LoginUserRequest>,
    session: Session,
) -> impl Responder {
    let session_value: Result<Option<Uuid>, SessionGetError> = session.get("user_id");

    let user_id = session_value.unwrap_or_default();

    if user_id.is_some() {
        log::info!("user login service user already login err");
        return HttpResponse::TemporaryRedirect().finish();
    }

    match login_user.validate() {
        Ok(_) => (),
        Err(err) => {
            log::info!("user login service validate error:{err}");
            return HttpResponse::BadRequest().finish();
        }
    }

    let user = UserRepository::login(&pool, login_user).await;

    match user {
        Ok(user) => {
            session.insert("user_id", user.id).unwrap();
            HttpResponse::Ok().finish()
        }
        Err(err) => {
            log::info!("user login service login error:{err}");
            HttpResponse::BadRequest().finish()
        }
    }
}

async fn register(
    pool: web::Data<PgPool>,
    new_user_info: web::Json<CreateUserRequest>,
    session: Session,
) -> impl Responder {
    let session_value: Result<Option<Uuid>, SessionGetError> = session.get("user_id");

    let user_id = session_value.unwrap_or_default();

    if user_id.is_some() {
        log::info!("user register service user already login err");
        return HttpResponse::TemporaryRedirect().finish();
    }

    match new_user_info.validate() {
        Ok(_) => (),
        Err(err) => {
            log::info!("user register service validate error:{err}");
            return HttpResponse::BadRequest().finish();
        }
    }

    let user = UserRepository::register(&pool, new_user_info).await;

    match user {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => {
            log::info!("user register service register error:{err}");
            HttpResponse::BadRequest().finish()
        }
    }
}

async fn logout(pool: web::Data<PgPool>, session: Session) -> impl Responder {
    let session_value: Result<Option<Uuid>, SessionGetError> = session.get("user_id");

    let user_id = session_value.unwrap_or_default();

    if user_id.is_none() {
        return HttpResponse::Unauthorized().finish();
    }

    let user_id = user_id.unwrap();

    let user = UserRepository::get_user_by_uuid(&pool, &user_id).await;

    match user {
        Some(_) => {
            session.remove("user_id");
            session.purge();
            HttpResponse::Ok().finish()
        }
        None => HttpResponse::Unauthorized().finish(),
    }
}
