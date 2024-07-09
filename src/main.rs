use axum::{
    routing::get,
    Router,
    Json,
    response::Html
};
use serde_json::{Value,json};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/",get(root))
        .route("/json", get(json))
        .route("/page", get(handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener,app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn json() -> Json<Value> {
    Json(json!({
        "data": 42
    }))
}

async fn handler() -> Html<&'static str> {
    Html("
        <header>
            <nav>
                <ul>
                    <li>Home</li>    
                </ul>
            </nav>
        </header>
        <h1>Hello,World!</h1>
        <main>
            <p>aaaa</p>
        </main>
    ")
}
