use hyper::service::{make_service_fn, service_fn};
use hyper::{body::Bytes, Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::time::{sleep, Duration};
use tokio::{pin, select};

async fn index_handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from(Bytes::from_static(
        include_bytes!("index.html"),
    ))))
}
async fn genkey_handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from(Bytes::from_static(
        include_bytes!("genkey.html"),
    ))))
}

#[tokio::main]
async fn main() {
    let main_task = async {
        // Construct our SocketAddr to listen on...
        let index_addr = SocketAddr::from(([0, 0, 0, 0], 80));
        eprintln!("{}", index_addr);
        // And a MakeService to handle each connection...
        let index_service =
            make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(index_handle)) });
        // Then bind and serve...
        let index_server = Server::bind(&index_addr).serve(index_service);
        println!("main_service");
        // And run forever...
        if let Err(e) = index_server.await {
            eprintln!("server error: {}", e);
        }
    };
    pin!(main_task);
    let background_task = async {
        sleep(Duration::from_millis(1_000)).await;
        // Construct our SocketAddr to listen on...
        let genkey_addr = SocketAddr::from(([0, 0, 0, 0], 81));
        eprintln!("{}", genkey_addr);
        // And a MakeService to handle each connection...
        let genkey_service =
            make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(genkey_handle)) });
        // Then bind and serve...
        let genkey_server = Server::bind(&genkey_addr).serve(genkey_service);
        println!("genkey_service");
        // And run forever...
        if let Err(e) = genkey_server.await {
            eprintln!("server error: {}", e);
        }
    };
    pin!(background_task);

    let mut main_task_complete = false;
    let mut background_task_complete = false;

    while !main_task_complete || !background_task_complete {
        select! {
            _ = &mut main_task, if !main_task_complete => {
                main_task_complete = true;
            }
            _ = &mut background_task, if !background_task_complete => {
                background_task_complete = true;
            }
        }
    }
}
