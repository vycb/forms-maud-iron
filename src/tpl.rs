use std::collections::HashMap;
use sqlite::{FileData};

pub fn getform<'a>(map: &'a HashMap<String,String>, rows: Box<Vec<FileData>>)-> Box<String> {
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
				    
				    #for i in 0..rows.len() as usize {
					    div {
							span $rows.get(i).unwrap().name
					    	img src={"image?id="$rows.get(i).unwrap().id}  /
						}
				    }
				}
			}
		}).unwrap();
	
	box buffer
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

