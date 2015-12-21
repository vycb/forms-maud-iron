use std::collections::HashMap;

pub fn getform<'a>(map: &'a HashMap<String,String>)-> String {
	let null = "".to_string();
	let mut buffer = String::new();
	let sq = if map.contains_key(&"sq".to_string()) { map.get(&"sq".to_string()).unwrap() }else{ &null };
	let fname = if map.contains_key(&"fname".to_string()) { map.get(&"fname".to_string()).unwrap() }else{ &null };
	html!(buffer, {
			html {
				head {
					title {
						"Form " $map["name"] "!"
					}
					meta charset="utf-8" /
						
				}
				body {
	    		p { "Form, " $map["name"] "!" }
			    h1 {"test of " $map["greating"] "!"}
			    
	    		form method="post" enctype="multipart/form-data" action="/form"{ 
						
						label for="sq" { "Search: " }
						
						input type="text" name="sq" id="sq" value=$sq /
						br /
						
						label for="fname" { "Name: " }
						
						input type="text" name="fname" id="fname" value=$fname /
						br /
						
						label for="image" { "Upload an image: " }
						input type="file" name="image" id="image" /
						br /
						
						input type="submit" value="Submit" id="submit" /
	    			
	    		}
			    
			    p {
			      "curl -sSf https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly"
						br /
						"curl -sSf https://static.rust-lang.org/rustup.sh | sh -s -- --channel=beta"
					}
				}
			}
		}).unwrap();
	
	buffer
}


pub fn gethtml<'a>(map: &'a HashMap<&str,&str>)-> String {
	let mut buffer = String::new();
	html!(buffer, {
			html {
				head {
					title {
						"Hello " $map["name"] "!"
					}
					meta charset="utf-8" /
						
				}
				body {
	    		p { "Hi, " $map["name"] "!" }
			    h1 {"test of " $map["greating"] "!"}
			    p {
			      "Watch as I work my gypsy magic"
						br /
						"Eye of a newt and cinnamon"
					}
				}
			}
		}).unwrap();
	
	buffer
}

