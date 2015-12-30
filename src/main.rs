#![feature(box_syntax)]
#![feature(plugin)]
#![plugin(maud_macros)]
extern crate maud;
use std::collections::HashMap;
extern crate rusqlite;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate iron;
extern crate maud_iron as mde;
extern crate params;
use params::{Params};
mod tpl;
use iron::prelude::*;
use iron::mime::Mime;
use iron::{status};
use mde::{Template, MaudEngine};
extern crate url;
mod requested_path;
mod form;
mod files;
mod sqlite;
use sqlite::{Client};
mod route;

fn image(req: &mut Request) -> IronResult<Response> {
	let pm = req.get_ref::<Params>().unwrap();

	let content_type = "image/jpeg".parse::<Mime>().unwrap();
	let sqlc = Client::new();
	let id = match pm["id"].clone() {
		params::Value::String(value) => value,
		_ => String::new()
	};
	let rows = sqlc.query(&id);
	let row = rows.get(0).unwrap().clone();
	let data = row.data.unwrap();
	let mut resp = Response::new();
	resp.set_mut(content_type)
	.set_mut((status::Ok, data));
	Ok(resp)

}

fn index(_: &mut Request) -> IronResult<Response> {
    let content_type = "text/html".parse::<Mime>().unwrap();
    let mut map = HashMap::new();
    map.insert("name", "Maud Iron plugin");
    map.insert("greating", "Great Maud Iron plugin!");
    let mut resp = Response::new();
 		resp.set_mut(content_type)
	 		.set_mut(Template::new(tpl::gethtml(&map)))
	 		.set_mut(status::Ok);
	Ok(resp)
}

fn main() {
	env_logger::init().unwrap();
	
	let mut chain = Chain::new(route::Router::init_routes());
	
	let mde = MaudEngine::new();
	chain.link_after(mde);
	
	Iron::new(chain).http("wram:8080").unwrap();
}




