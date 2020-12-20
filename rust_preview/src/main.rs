use hyper::{Server, Request, Response, Error, StatusCode, Method, Body};
use hyper::service::{make_service_fn, service_fn};

type Rx = Result<Response<Body>, Error>;

const HOMEPAGE: &'static str = r#"
    <!doctype html>
        <head>
            <title> Rust code</title>
        </head>
        <body>
            <h3> Sample rust code </h3>
        </body>
"#;

async fn handler(req: Request<Body>) -> Rx {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            Ok(Response::new(HOMEPAGE.into()))
        }
        _ => {
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap();
            
            Ok(response)
        }
    }
}

#[tokio::main]
async fn main() {
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));

    let make_service_call = make_service_fn(|_conn| async {
        Ok::<_, Error>(service_fn(handler))
    });

    let server = Server::bind(&addr).serve(make_service_call);

    if let Err(e) = server.await {
        eprintln!("Server responded error: {}", e);
    }
}
