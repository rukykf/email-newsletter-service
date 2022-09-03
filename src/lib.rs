pub mod configuration;
pub mod routes;
pub mod startup;

pub use startup::*;

use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
