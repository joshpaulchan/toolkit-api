use std::env;
use warp::Filter;

#[tokio::main]
async fn main() {
    let port: u16 = env::var("PORT")
        .map(|val| val.parse().unwrap())
        .unwrap_or(3030);
    // let port: u16 = 3030;

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let routes = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));
    // Match any request and return hello world!
    // let routes = warp::any().map(|| "Hello, World!");

    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}
