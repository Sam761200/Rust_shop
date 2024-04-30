use warp::{Filter, Reply, Rejection};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Product {
    id: i32,
    name: String,
    price: f64,
    description: String,
}

async fn get_products() -> Result<impl Reply, Rejection> {
    // Pour l'instant, retournons des données statiques.
    let products = vec![
        Product { id: 1, name: "Laptop".to_string(), price: 999.99, description: "A high-end laptop.".to_string() },
        Product { id: 2, name: "Phone".to_string(), price: 699.99, description: "A new smartphone.".to_string() },
    ];

    Ok(warp::reply::json(&products))
}

pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
    warp::path("products")
        .and(warp::get())
        .and_then(get_products)
}
