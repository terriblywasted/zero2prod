use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

async fn greet(req: HttpRequest) -> impl Responder {
  let name = req.match_info().get("name").unwrap_or("World");
  format!("Hello {}!", &name)
}

async fn health_check(_req: HttpRequest) -> impl Responder {
  HttpResponse::Ok()
}

#[derive(serde::Deserialize)]
struct FormData {
  email: String,
  name: String,
}

async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
  println!("{} : {}", _form.name, _form.email);
  HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
  let server = HttpServer::new(|| {
    App::new()
      .route("/", web::get().to(greet))
      .route("/health_check", web::get().to(health_check))
      .route("/subscriptions", web::post().to(subscribe))
  })
  .listen(listener)?
  .run();

  Ok(server)
}
