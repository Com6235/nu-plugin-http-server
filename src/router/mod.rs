use axum::{body::Body, http::Response as AResponse, response::IntoResponse, routing::get, Router as ARouter};

pub mod server;

#[derive(Clone)]
pub struct Route {
    pub route: String,
    data: Vec<u8>,
    pub mime: String
}

#[allow(unused)]
impl Route {
    pub fn new(route: String, data: Vec<u8>, mime: String) -> Route {
        Route { route, data, mime }
    }

    pub fn from_string(route: String, data: String, mime: String) -> Route {
        Route { route, data: data.as_bytes().iter().map(|x| *x).collect(), mime }
    }

    pub fn from_str(route: &str, data: &str, mime: String) -> Route {
        Route { route: String::from(route), data: data.as_bytes().iter().map(|x| *x).collect(), mime }
    }

    pub fn build_router(&self, router: ARouter) -> ARouter {
        let (data, mime) = (self.data.clone(), self.mime.clone());
        router.route(
            self.route.as_str(),
            get(async || Response::new(data, mime))
        )
    }
}

pub struct Response {
    data: Vec<u8>,
    mime: String,
}

impl Response {
    pub fn new(data: Vec<u8>, mime: String) -> Response {
        Response { data, mime }
    } 
}

impl IntoResponse for Response {
    fn into_response(self) -> AResponse<axum::body::Body> {
        let resp = AResponse::builder()
            .header("Content-Type", self.mime)
            .header("Content-Length", self.data.len())
            .body(Body::from(self.data));

        resp.unwrap()
    }
}