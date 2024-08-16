use hyper::service::{make_service_fn, service_fn};
use hyper::{body::Bytes, Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn index_handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from(Bytes::from_static(
        include_bytes!("genkey.html"),
    ))))
}

#[tokio::main]
async fn main() {
    // Construct our SocketAddr to listen on...
    let index_addr = SocketAddr::from(([0, 0, 0, 0], 81));
    eprintln!("{}", index_addr);

    // And a MakeService to handle each connection...
    let index_service = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(index_handle)) });

    // Then bind and serve...
    let index_server = Server::bind(&index_addr).serve(index_service);

    // And run forever...
    if let Err(e) = index_server.await {
        eprintln!("server error: {}", e);
    }
}
