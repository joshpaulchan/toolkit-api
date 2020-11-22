#![deny(warnings)]
use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let routes = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));
    // Match any request and return hello world!
    // let routes = warp::any().map(|| "Hello, World!");

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
