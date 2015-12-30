use iron::prelude::*;
use iron::mime::Mime;
use iron::{status};
extern crate params;
use params::Params;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use tpl;
use mde::{Template};
use sqlite::{Client, FileData};

pub fn form(req: &mut Request) -> IronResult<Response> {
	let mut resp = Response::new();
	debug!("{:?}", req.get_ref::<Params>());
	let pm = req.get_ref::<Params>().unwrap();
	let mut map: HashMap<String, String> = HashMap::new();
	let sqlc = Client::new();
	
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
				let filename = value.filename().unwrap().to_string();
				let mimetype = value.content_type().to_string();
				let mut file = File::open(path).unwrap();
				let mut contents: Vec<u8> = Vec::new();
			    file.read_to_end(&mut contents).unwrap();
				
				let fd = FileData {
					id: 1,
					name: filename,
					mimetype: mimetype,
					data: Some(contents)
				};
				
				sqlc.insert(&fd);
				
				debug!("path: {}", path);
				debug!("filename {}",  value.filename().unwrap() );
				
		    },
		    params::Value::Array(ref value) => debug!("Array:{:?}", value),
		    params::Value::Map(ref value) => debug!("Map:{:?}", value),
		}
	}
	
	let content_type = "text/html".parse::<Mime>().unwrap();
	
	let rows = sqlc.query("%");
	
	map.insert("name".to_string(), "Maud Iron plugin".to_string());
	map.insert("greating".to_string(), "Great Maud Iron plugin!".to_string());
	map.insert("image".to_string(), "orange.jpeg".to_string());

 	resp.set_mut(content_type)
		.set_mut(Template::new(tpl::getform(&map, rows)))
		.set_mut(status::Ok);
	Ok(resp)
}