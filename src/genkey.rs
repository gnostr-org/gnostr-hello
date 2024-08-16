use hyper::service::{make_service_fn, service_fn};
use hyper::{body::Bytes, Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn genkey_handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from(Bytes::from_static(
        include_bytes!("genkey.html"),
    ))))
}

#[tokio::main]
async fn main() {
    // Construct our SocketAddr to listen on...
    let genkey_addr = SocketAddr::from(([0, 0, 0, 0], 81));
    eprintln!("{}", genkey_addr);

    // And a MakeService to handle each connection...
    let genkey_service =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(genkey_handle)) });

    // Then bind and serve...
    let genkey_server = Server::bind(&genkey_addr).serve(genkey_service);

    // And run forever...
    if let Err(e) = genkey_server.await {
        eprintln!("server error: {}", e);
    }
}
