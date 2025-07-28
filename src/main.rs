#![allow(unused)]

use std::fmt::Display;

use actix_web::{get, App, HttpResponse, HttpServer, Responder, Result};
use maud::{Markup, html};
use crate::{api::{get_by_diva, get_by_stop_id}, components::{layout, line_card::line_card}, csv::load_csv_data};

mod csv;
mod api;
mod components;

#[derive(Debug)]
struct ErrorOwO {
    err: anyhow::Error,
}

impl From<anyhow::Error> for ErrorOwO {
    fn from(value: anyhow::Error) -> Self {
        ErrorOwO { err: value }
    }
}

impl Display for ErrorOwO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.err)
    }
}

impl actix_web::error::ResponseError for ErrorOwO {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::new(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
    }
}


#[get("/")]
async fn index() -> Result<Markup> {
    let res = html! {
        html {
            body {
                h1 { "Hello, World!" }
            }
        }
    };

    Ok(res)
}

#[get("/a")]
async fn stop() -> Result<Markup> {
    let res = get_by_stop_id(15).await.map_err(ErrorOwO::from)?;

    let line = &res.data.monitors[0].lines[0];
    let html = layout(line_card(line));

    Ok(html)
}

#[get("/b")]
async fn bla() -> Result<String> {
    let res = get_by_stop_id(15).await.map_err(ErrorOwO::from);

    match res {
        Ok(res) => println!("{res:#?}"),
        Err(err) => println!("{err:#?}")
    }

    Ok("fuck :D".to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (lines, stops, routes) = load_csv_data().expect("should work grrr");

    println!("Starting the most important web-server known to manunkind");

    HttpServer::new(|| App::new()
        .service(stop)
        .service(index)
        .service(bla)
    )
        .bind(("localhost", 6969))?
        .run()
        .await
}
