use warp::{filters::BoxedFilter, Filter};

fn path_prefix() -> BoxedFilter<()> {
    warp::path("hello").boxed()
}

pub fn hello() -> BoxedFilter<(String,)> {
    let param = warp::path::param::<String>();

    warp::get().and(path_prefix()).and(param).boxed()
}
