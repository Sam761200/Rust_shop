mod routes;

#[tokio::main]
async fn main() {
    let api_routes = routes::routes();

    warp::serve(api_routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
