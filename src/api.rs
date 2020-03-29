#[macro_export]
macro_rules! health_check {
    () => {
        routes::health_check().and_then(handlers::health_check)
    };
}

#[macro_export]
macro_rules! create_transfer {
    () => {
        routes::create().and_then(handlers::create)
    };
}
