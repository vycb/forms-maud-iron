use iron::prelude::*;
use iron::modifiers::Redirect;
extern crate params;
use std::fs;
use std::path::Path;
use std::error::Error;
use std::fmt;
extern crate url;
use requested_path::RequestedPath;
#[derive(Copy, Clone)]
pub struct OriginalUrl;
use iron::typemap;
use iron::{status, Url};
impl typemap::Key for OriginalUrl { type Value = Url; }


pub fn files(req: &mut Request) -> IronResult<Response> {
	use std::io;
	let requested_path = RequestedPath::new(Path::new(""), req);
	
	 let metadata = match fs::metadata(&requested_path.path) {
	    Ok(meta) => meta,
	    Err(e) => {
	        let status = match e.kind() {
	            io::ErrorKind::NotFound => status::NotFound,
	            io::ErrorKind::PermissionDenied => status::Forbidden,
	            _ => status::InternalServerError,
	        };
	
	        return Err(IronError::new(e, status))
	    },
	};
	 
  	// If the URL ends in a slash, serve the file directly.
    // Otherwise, redirect to the directory equivalent of the URL.
    if requested_path.should_redirect(&metadata, req) {
        // Perform an HTTP 301 Redirect.
        let mut redirect_path = match req.extensions.get::<OriginalUrl>() {
            None => &req.url,
            Some(original_url) => original_url,
        }.clone();

        // Append the trailing slash
        //
        // rust-url automatically turns an empty string in the last
        // slot in the path into a trailing slash.
        redirect_path.path.push("".to_string());

        return Ok(Response::with((status::MovedPermanently,
                                  format!("Redirecting to {}", redirect_path),
                                  Redirect(redirect_path))));
    }

	match requested_path.get_file(&metadata) {
        // If no file is found, return a 404 response.
        None => Err(IronError::new(NoFile, status::NotFound)),
        Some(path) => {
            let path: &Path = &path;
            Ok(Response::with((status::Ok, path)))
        },
    }
}

#[derive(Debug)]
pub struct NoFile;

impl Error for NoFile {
    fn description(&self) -> &str { "File not found" }
}

impl fmt::Display for NoFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}
