use warp::Filter;
use warp::Reply;
use warp::reply::Response;
use warp::reject::Reject;

#[tokio::main]
async fn main() {
    let hello_route = warp::path("hello");
    let hello_to = hello_route
        .and(warp::path::param())
        .then(hello);

    let hello_nobody = hello_route.
        then(|| hello(String::new()));

    warp::serve(
        hello_to.or(hello_nobody)
    )
        .run(([0, 0, 0, 0], 5149))
        .await;
}

#[derive(Debug)]
struct EmptyName;
impl Reject for EmptyName {}
impl Reply for EmptyName {
    fn into_response(self) -> Response {
        Response::new("Hello to nobody!".into())
    }
}

async fn hello(name: String) -> Result<String, EmptyName> {
    if name.len() == 0 {
        Err(EmptyName)
    }
    else {
        Ok(format!("Hello {name}"))
    }
}
