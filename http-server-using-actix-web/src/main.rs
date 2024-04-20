use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
	HttpResponse::Ok().body("Hello world!\n")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
	HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let server = HttpServer::new(|| App::new().service(hello).service(echo));
	server.bind(("127.0.0.1", 8080))?.run().await
}
