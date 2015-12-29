#![feature(plugin)]
#![plugin(maud_macros)]
extern crate maud;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate iron;
extern crate maud_iron as mde;
extern crate params;
use std::collections::HashMap;
mod tpl;
use iron::prelude::*;
use iron::mime::Mime;
use iron::{Handler,status};
use params::Params;
use mde::{Template, MaudEngine};
use std::io::prelude::*;
use std::fs::File;

fn form(req: &mut Request) -> IronResult<Response> {
	let mut resp = Response::new();
	debug!("{:?}", req.get_ref::<Params>());
	let pm = req.get_ref::<Params>().unwrap();
	let mut map: HashMap<String, String> = HashMap::new();
	
	for (key, val) in pm.iter() {
		debug!("key:{}", key);
		match *val {
		    params::Value::Null => debug!("{}", "null"),
		    params::Value::Boolean(value) => debug!("bool:{:?}", value),
		    params::Value::I64(value)  => debug!("i64:{:?}", value),
		    params::Value::U64(value)  => debug!("u64:{:?}", value),
		    params::Value::F64(value)  => debug!("f64:{:?}", value),
		    params::Value::String(ref value) => {debug!("String:{}", value);
		    	map.insert(key.clone(), value.clone());
		    },
		    params::Value::File(ref value) => { debug!("File:{:?}", value); 
					let path = value.path().to_str().unwrap();
					let filename = value.filename().unwrap();
						
					match std::fs::copy(path, filename ) {
						Ok(n)  => debug!("File upload Ok size: {}", n),
    				Err(e) => debug!("File upload Error: {}", e),
					}
					debug!("path: {}", path);
					debug!("filename {}",  value.filename().unwrap() );
					
		    },
		    params::Value::Array(ref value) => debug!("Array:{:?}", value),
		    params::Value::Map(ref value) => debug!("Map:{:?}", value),
		}
	}
	
	let content_type = "text/html".parse::<Mime>().unwrap();
	
	map.insert("name".to_string(), "Maud Iron plugin".to_string());
	map.insert("greating".to_string(), "Great Maud Iron plugin!".to_string());
	map.insert("image".to_string(), "orange.jpeg".to_string());

 	resp.set_mut(content_type)
		.set_mut(Template::new(tpl::getform(&map)))
		.set_mut(status::Ok);
	Ok(resp)
}


fn image(_: &mut Request) -> IronResult<Response> {
    let content_type = "image/jpeg".parse::<Mime>().unwrap();
    
    let mut f = File::open("orange.jpg").unwrap();
	let mut s = String::new();
	f.read_to_string(&mut s).unwrap();
	
    let mut resp = Response::new();
 		resp.set_mut(content_type)
	 		.set_mut(s.as_bytes())
	 		.set_mut(status::Ok);
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
	
	router.add_route("form".to_string(), form);
	
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
        match self.routes.get(&req.url.path.join("/")) {
            Some(handler) => handler.handle(req),
            None => Ok(Response::with(status::NotFound))
        }
    }
}
