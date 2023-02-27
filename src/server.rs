use actix_web::{get, web, App, HttpRequest, HttpServer, Responder};

#[get("/hello/{world}")]
async fn hello_world(world: web::Path<String>) -> impl Responder {
    format!("Hello {world}")
}

#[get("/friend/request")]
async fn friend_request(req: HttpRequest) -> impl Responder {
    println!("req.peer_adr {:#?}", req.peer_addr());
    println!("req.connection_info {:#?}", req.connection_info());
    format!("req {:#?}", req)
}

pub async fn run_server(port: u16) {
    tokio::spawn(
        HttpServer::new(|| App::new().service(hello_world).service(friend_request))
            .bind(("127.0.0.1", port))
            .unwrap()
            .run(),
    );
}
