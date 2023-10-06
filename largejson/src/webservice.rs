use std::{
    path::Path,
    convert::Infallible,
};

use figment::{Figment, providers::{Yaml, Format}};
use futures::{Stream, StreamExt};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio_util::{sync::CancellationToken, bytes::Buf};
use warp::{hyper::StatusCode, Rejection, Reply, reply::json, reject::Reject, Filter};

use crate::{
    NAME,
    schema::{schema_string, Person, validate},
    tokio_tools::run_in_tokio,
    error::MyError,
};


#[derive(Deserialize, Debug, Clone)]
pub struct WebServicePrefixConfig {
    pub name: String,
    pub version: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct WebServiceConfig {
    /// Prefix of the served API
    pub prefix: WebServicePrefixConfig,
}

#[derive(Deserialize, Debug)]
pub struct MyConfig {
    /// Config of my web service
    pub webservice: WebServiceConfig,
}

impl MyConfig {
    // Note the `nested` option on both `file` providers. This makes each
    // top-level dictionary act as a profile.
    pub fn figment<P: AsRef<Path>>(path: P) -> Figment {
        Figment::new().merge(Yaml::file(path))
    }
}

#[derive(Serialize)]
struct ValidationReply {
    size: usize,
    length: usize,
    validate: bool,
}

async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let (code, json_message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, json(&"Not Found".to_string()))
    } else if err.find::<warp::reject::PayloadTooLarge>().is_some() {
        (StatusCode::BAD_REQUEST, json(&"Payload too large".to_string()))
    } else if let Some(e) = err.find::<MyError>() {
        match e {
            MyError::Message(_) => todo!(),
            MyError::Cancelled => todo!(),
            MyError::Serde(_) => todo!(),
            MyError::Io(_) => todo!(),
            MyError::JsonValidation(errors) => {
                let myval = serde_json::json!( { "status:": "validation failed","errors": errors});

                (StatusCode::BAD_REQUEST, json(&myval))
            },
            MyError::ValidationError() => todo!(),
        }
    } else {
        eprintln!("unhandled error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            json(&"Internal Server Error".to_string()),
        )
    };

    Ok(warp::reply::with_status(json_message, code))
}

async fn receive_binary_review(
    mut body: impl Stream<Item = Result<impl Buf, warp::Error>> + Unpin + Send + Sync
) -> Result<impl Reply, Rejection> {
    // https://github.com/seanmonstar/warp/issues/448

    let error_limit = 20;

    let mut file_content = Vec::new();

    let mut chunk_tot = 0;
    while let Some(buf) = body.next().await {
        let mut buf = buf.unwrap();
        while buf.remaining() > 0 {
            let chunk = buf.chunk();
            let chunk_len = chunk.len();
            // println!("getting chunk of len = {chunk_len}");
            chunk_tot += chunk_len;
            file_content.extend_from_slice(chunk);
            buf.advance(chunk_len);
        }
    }

    let schema_str = schema_string::<Vec<Person>>()?;

    match validate(&schema_str,  &file_content[..], error_limit) {
        Ok(data) => {

            let length = if let Value::Array(array) = &data {
                array.len()
            } else {
                0
            };

            let reply = ValidationReply{
                size: chunk_tot,
                length: length,
                validate: true,
            };

            Ok(warp::reply::json(&reply))
        },
        Err(err) => Err(err.into()),
    }
}

// Marker trait to indicate MyError is a planned rejection type
impl Reject for MyError {}

pub async fn http_service_cancellable(ct: CancellationToken, config: WebServiceConfig) -> Result<(), MyError> {

    // Setup http server
    let download_route = warp::path("files")
        .and(warp::fs::dir("./files/"));

    let upload_route_review = warp::path("review")
        .and(warp::post())
        .and(warp::body::stream())
        .and_then(receive_binary_review);

    let weblog = warp::log(NAME);

    let combined = download_route
        .or(upload_route_review)
        .recover(handle_rejection)
        .with(weblog);

    let prefix_path = warp::path(config.prefix.name.clone())
        .and(warp::path(config.prefix.version.clone()));

    let router = prefix_path
        .and(combined);

    let (addr, server) = warp::serve(router)
        .bind_with_graceful_shutdown(
            ([0, 0, 0, 0], 8080),
            async move {ct.cancelled().await},
        );
    info!("Server started on port {}", addr);

    Ok(server.await)
}

pub fn http_receive_json(config: WebServiceConfig) -> Result<(), MyError> {

    let ct = CancellationToken::new();

    run_in_tokio(http_service_cancellable(ct, config))
}
