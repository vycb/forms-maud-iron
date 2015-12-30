use rusqlite::{Connection};

pub struct Client {
	conn: Connection
}


impl Client {
	pub fn new() -> Client {
		Client{
			conn: Connection::open("db/db.sqlite").unwrap_or_else(|e|{ panic!("Connection error:{}", e) } )
		}
	}
    
	pub fn insert(&self, fd: &FileData) {
		
		self.conn.execute("CREATE TABLE IF NOT EXISTS files (
		id			INTEGER PRIMARY KEY,
		name		TEXT NULL,
		mimetype	TEXT NULL,
		data		BLOB NULL
		)", &[]).unwrap();
		
		self.conn.execute("INSERT INTO files (name, mimetype, data)
			VALUES ($1, $2, $3)",
			&[&fd.name, &fd.mimetype, &fd.data]).unwrap();
	}
	
	pub fn query<'a>(&self, key: &str) -> Box<Vec<FileData>> {
		let mut stmt = self.conn.prepare("SELECT id, name, mimetype, data FROM files WHERE id LIKE $1").unwrap();
	    
	    let rows = stmt.query_map(&[&key], |row| {
	        FileData {
	            id: row.get::<i32>(0),
	            name: row.get::<String>(1),
	            mimetype:row.get::<String>(2),
	            data: row.get::<Option<Vec<u8>>>(3)
	        }
	    }).unwrap();
	    
	    let mut vec = Vec::new();
	    for row in rows {
	    	vec.push(row.unwrap());
	    }
	    
	    box vec
		
	}

}


#[derive(Debug, Clone)]
pub struct FileData {
    pub id: i32,
    pub name: String,
    pub mimetype: String,
    pub data: Option<Vec<u8>>
}


