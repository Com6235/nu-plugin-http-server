use std::{io::Error, net::SocketAddr};

use axum::Router;
use tokio::{net::TcpListener, task::JoinHandle};

use super::Route;

pub struct Server {
    bind_addr: String,
    routes: Vec<Route>,
    binded_addr: Option<SocketAddr>,
}

impl Server {
    pub fn new(bind_addr: String, routes: Vec<Route>) -> Server {
        Server { bind_addr, routes, binded_addr: None, }
    }

    pub async fn start(&self) -> JoinHandle<Result<(), Error>> {
        let listener = TcpListener::bind(self.bind_addr.clone())
            .await
            .unwrap();

        let mut app = Router::new();
        for route in &self.routes {
            app = route.build_router(app)
        }

        tokio::spawn(async { axum::serve(listener, app).into_future().await })
    }

    pub fn binded_address(&self) -> Option<SocketAddr> {
        self.binded_addr
    }

    pub fn routes(&self) -> Vec<Route> {
        self.routes.clone()
    }
}