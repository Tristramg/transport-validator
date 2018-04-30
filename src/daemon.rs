extern crate failure;
extern crate gotham;
extern crate gtfs_structures;
extern crate mime;

use hyper::{Response, StatusCode};
use gotham::router::Router;
use gotham::router::builder::*;
use gotham::state::{FromState, State};
use gotham::http::response::create_response;
use validators::validate;
use std::env;

#[derive(Deserialize, StateData, StaticResponseExtender)]
struct QueryStringExtractor {
    url: String,
}

fn validation_handler(mut state: State) -> (State, Response) {
    let query_param = QueryStringExtractor::take_from(&mut state);

    let res = match validate(&query_param.url) {
        Ok(json) => create_response(
            &state,
            StatusCode::Ok,
            Some((json.into_bytes(), mime::APPLICATION_JSON)),
        ),
        Err(err) => create_response(
            &state,
            StatusCode::InternalServerError,
            Some((
                format!("{{\"error\": \"{}\"}}", err).into_bytes(),
                mime::APPLICATION_JSON,
            )),
        ),
    };

    (state, res)
}

fn router() -> Router {
    build_simple_router(|route| {
        route
            .get("/validate")
            .with_query_string_extractor::<QueryStringExtractor>()
            .to(validation_handler);
    })
}

pub fn run_server() {
    let port = env::var("PORT").unwrap_or("7878".to_string());
    let bind = env::var("BIND").unwrap_or("127.0.0.1".to_string());
    let addr = format!("{}:{}", bind, port);
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}