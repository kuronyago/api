// use warp::Filter;

#[macro_export]
macro_rules! get_hello_macro {
    () => {
        routes::hello().and_then(handlers::get_hello)
    };
}
