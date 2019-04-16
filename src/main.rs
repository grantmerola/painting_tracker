// This is how to use the db at least select hopefully insert should be easier 
// just proof of concept  and opening for starting lib

use mysql;
use mysql::Value;
//use painting_tracker::*;

// mirror of our test db
#[derive(Debug)]
struct Book {
	bookid: i32,
	title: String,
	seriesid: i32,
	authorid: i32
}

fn main() {
    //specify server info
    let connection_url = "mysql://root@localhost:3306/test";
	// set up what i believe is a pool of connections to the sql server
	let pool = mysql::Pool::new(connection_url);

	let pool = match pool {
		Ok(poolcon) => poolcon,
		Err(error) => {
			panic!("panicked while trying to connect to server: {:?}", error)
		},
	};
	// our vector of what will be rows
	let mut list_of_books = Vec::new();
	//black magic that then iterates over our rows but In a special Row type that is useless 
	for row in pool.prep_exec(r"SELECT * FROM books;",()).unwrap() {

	 	// rust likes to wrap things so we take them out and  stuff it in a Vec of another useless type Value
	 	let rows: Vec<Value> = row.unwrap().unwrap();
	 	
	 	//add book to list
	 	list_of_books.push(
	 		Book{
	 			// turn it to a string to get it out of weird types and parse into useful values
	 			bookid: rows[0].as_sql(false).parse().unwrap(), 
	 			title: rows[1].as_sql(false).parse().unwrap(), 
	 			seriesid: rows[2].as_sql(false).parse().unwrap(), 
	 			authorid: rows[3].as_sql(false).parse().unwrap()
	 		}
	 	);	 	
	 };
	 //proof it was done
	 //println!("this is end vector of books: {:?}", list_of_books );

	println!("---------RAN---------");
}
