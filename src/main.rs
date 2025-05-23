use actix_web::{web, App, HttpRequest, HttpServer, Responder};

/// Responder -> Trait implemented by return types that Actix can convert into replies
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    /// Httpserver is a transport layer which checks the address to listen on?
    HttpServer::new(|| {
        // App is the logic layer which contains details about the routes,middleware and handlers. It is responsible for matching incoming requests to routes. 
        App::new()
            // web::get() -> it is a guard  which checks the condition the request must satisfy
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    // An endpoint is a combination of the url , an handler function nad the optional set of conditions. 
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
