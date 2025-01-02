use actix_cors::{self, Cors};

pub fn get_cors_config() -> Cors {
    let client = String::from("http://localhost");
    let methods = vec!["GET", "POST"];

    Cors::default()
        .allowed_origin(&client)
        .allowed_methods(methods)
        .max_age(None)
        .supports_credentials()
}
