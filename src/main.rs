#![feature(box_syntax)]
#![feature(plugin)]
#![plugin(maud_macros)]
extern crate maud;
extern crate rusqlite;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate iron;
extern crate maud_iron as mde;
extern crate params;
use params::{Params};
use std::collections::HashMap;
mod tpl;
use iron::prelude::*;
use iron::mime::Mime;
use iron::{Handler,status};
use mde::{Template, MaudEngine};
extern crate url;
mod requested_path;
mod form;
mod files;
mod sqlite;
use sqlite::{Client};


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
	
	let mut router = Router::new();
	
	router.add_route("".to_string(), index);
	
	router.add_route("form".to_string(), form::form);

	router.add_route("files".to_string(), files::files);
	router.add_route("image".to_string(), image);
	
	router.add_route("hello/again".to_string(), |_: &mut Request| {
	   Ok(Response::with("Hello again !"))
	});
	
	router.add_route("error".to_string(), |_: &mut Request| {
	   Ok(Response::with(status::BadRequest))
	});
	
	let mut chain = Chain::new(router);
	
	let mde = MaudEngine::new();
	chain.link_after(mde);
	
	Iron::new(chain).http("wram:8080").unwrap();
}

struct Router {
    routes: HashMap<String, Box<Handler>>
}

impl Router {
    fn new() -> Self {
        Router { routes: HashMap::new() }
    }

    fn add_route<H>(&mut self, path: String, handler: H) where H: Handler {
        self.routes.insert(path, Box::new(handler));
    }
}

impl Handler for Router {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
//		let fp = req.url.path.join("/");
    	let p1 = req.url.path[0].clone();
        match self.routes.get(&p1) {
            Some(handler) => handler.handle(req),
            None => Ok(Response::with(status::NotFound))
        }
    }
}



