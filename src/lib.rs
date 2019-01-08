#![feature(plugin)]
#![feature(proc_macro_hygiene, decl_macro)]
extern crate handlebars;
#[macro_use]
extern crate log;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

pub mod service;
pub mod routes;
mod handlebars_ext;
mod templates;