use axum::{extract::Query, response::Html, routing::{get, post, put, delete, MethodRouter}, Router};
use serde::Deserialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {

    
    let app = Router::new()
    .merge(root())
    .merge(get_foo())
    .merge(post_foo());

    // Address that server will bind to.
    let addr = SocketAddr::from(([127,0,0,1], 3000));
    //println!("listening on {}", addr.local_addr().unwrap());
     // Use `hyper::server::Server` which is re-exported through `axum::Server` to serve the app.
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// `Deserialize` need be implemented to use with `Query` extractor.
// #[derive(Deserialize)]
// struct RangeParameters {
//     start: usize,
//     end: usize,
// }

//handler process the request and return the response
// async fn handler(Query(range): Query<RangeParameters>) -> Html<String> {
//     // Generate a random number in range parsed from query.
//     let random_number = thread_rng().gen_range(range.start..range.end);

//     // Send response in html format.
//     Html(format!("<h1>Random Number: {}</h1>", random_number))
// }

#[derive(Deserialize)]
struct Users {
    name: String,
    phone: String,
}


fn root() -> Router {
    async fn handler() -> &'static str {
        "hello, world!"
    }

    route("/", get(handler))
}

fn get_foo() -> Router {
    async fn handler() -> &'static str {
        "Hi from 'GET /foo'"
    }

    route("/foo", get(handler))

}

fn post_foo() -> Router {
    async fn handler() -> &'static str {
        "Hi from 'POST /foo'"
    }

    route("/foo", post(handler))
}

fn route(path: &str, method_router: MethodRouter<()>) -> Router {
    Router::new().route(path, method_router)
}