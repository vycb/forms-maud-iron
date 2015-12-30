use std::collections::HashMap;
use form;
use files;
use iron::prelude::*;
use iron::{Handler,status};

pub struct Router {
    routes: HashMap<String, Box<Handler>>
}

impl Router {
	
    pub fn init_routes() -> Self{
        let mut router = Router::new();
		        	
		router.add_route("".to_string(), super::index);
		
		router.add_route("form".to_string(), form::form);
		
		router.add_route("files".to_string(), files::files);
		router.add_route("image".to_string(), super::image);
		
		router.add_route("hello/again".to_string(), |_: &mut Request| {
		   Ok(Response::with("Hello again !"))
		});
		
		router.add_route("error".to_string(), |_: &mut Request| {
		   Ok(Response::with(status::BadRequest))
		});
		
		router
    }
	
    fn new() -> Self {
        Router { routes: HashMap::new() }
    }

    fn add_route<H>(&mut self, path: String, handler: H) where H: Handler {
        self.routes.insert(path, Box::new(handler));
    }
}

impl Handler for Router {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
    	let p1 = req.url.path[0].clone();
        match self.routes.get(&p1) {
            Some(handler) => handler.handle(req),
            None => Ok(Response::with(status::NotFound))
        }
    }
}