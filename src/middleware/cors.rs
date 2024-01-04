use actix_cors::Cors;

pub fn cors() -> Cors {
    Cors::default()
        .allow_any_origin()
        .send_wildcard()
        .allow_any_method()
        .allow_any_header()
        .max_age(3600)
}
