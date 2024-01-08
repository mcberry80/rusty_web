use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/withtail/{tail:.*}")]
async fn with_tail(req: HttpRequest) -> impl Responder {
    let tail = req.match_info().get("tail").unwrap_or("World");
    //split the tail into tokens, separated by a slash
    let tokens: Vec<&str> = tail.split("/").collect();
    
    let mut result = String::from("<html><head><title>Hello!</title></head><body><h1>Hello world!</h1><ul> ");    for token in tokens {
        //build string like <li>token</li>
        result.push_str("<li>");
        result.push_str(token);
        result.push_str("</li>");
    }
    result.push_str("</ul></body></html>");
    HttpResponse::Ok().body(result)
    
}

#[get("/styledhello")]
async fn styled_hello() -> impl Responder {
    HttpResponse::Ok().body("<html><head><title>Hello!</title></head><body><h1>Hello world!</h1></body></html>")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default()) // Add this line
            .service(hello)
            .service(echo)
            .service(styled_hello)
            .service(with_tail)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
