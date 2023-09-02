# Controller Definition Macro
## Example
```rust
use std::{collections::HashMap, sync::RwLock, net::SocketAddr};

use axum::{routing::{MethodRouter, get, post, put, delete}, body::Body, Json, extract::Path, Router};
use controller_def::controller_def;
use lazy_static::lazy_static;

lazy_static! {
    static ref HASHMAP: RwLock<HashMap<String, String>> = RwLock::new(HashMap::new());
}

controller_def! {
    dict@"/dict" => MethodRouter<(), Body>;
    GET = || async move {
        let read = HASHMAP.read().unwrap();
        let res = Vec::from_iter(read.iter().map(|(k, v)| { [ k, v ] }));
        Json(serde_json::json!({ "data" : res }))
    };
    GET "/:id" = |Path(id): Path<String>| async move {
        let read = HASHMAP.read().unwrap();
        let res = read.get(&id);
        Json(serde_json::json!({ "data" : res }))
    };
    DELETE "/:id" = |Path(id): Path<String>| async move {
        let mut write = HASHMAP.write().unwrap();
        write.remove(&id);
        Json(serde_json::json!({ "message" : "ok" }))
    };
    POST "/:id" = |Path(id): Path<String>, Json(data): Json<String>| async move {
        let mut write = HASHMAP.write().unwrap();
        write.insert(id, data);
        Json(serde_json::json!({ "message" : "ok" }))
    };
    PUT "/:id" = |Path(id): Path<String>, Json(data): Json<String>| async move {
        let mut write = HASHMAP.write().unwrap();
        write.insert(id, data);
        Json(serde_json::json!({ "message" : "ok" }))
    };
}

#[tokio::main]
async fn main() {
    let app = Router::new();
    let app = dict().iter().fold(app, |app, (path, handler)| {
        app.route(&path, handler.clone())
    });
    axum::Server::bind(&SocketAddr::from(([127,0,0,1], 3000)))
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```