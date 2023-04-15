use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

async fn ping() -> HttpResponse {
    return HttpResponse::Ok().finish();
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let app = || {
        return App::new().route("/ping", web::get().to(ping));
    };

    let server = HttpServer::new(app).listen(listener)?.run();

    return Ok(server);
}
