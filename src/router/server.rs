use std::{io::Error, net::SocketAddr};

use axum::Router;
use tokio::task::JoinHandle;

use super::Route;

pub struct Server {
    pub bind_addr: String,
    pub routes: Vec<Route>,
}

impl Server {
    pub fn new(bind_addr: String, routes: Vec<Route>) -> Server {
        Server { bind_addr, routes }
    }

    pub async fn start(self) -> (JoinHandle<Result<(), Error>>, SocketAddr) {
        let listener = tokio::net::TcpListener::bind(self.bind_addr)
            .await
            .unwrap();
        let local_addr = listener.local_addr().unwrap();

        let mut app = Router::new();

        for route in self.routes {
            app = route.build_router(app)
        }

        let thread = tokio::spawn(async { axum::serve(listener, app).into_future().await });

        (thread, local_addr)
    }
}