// This is how to use the db at least select hopefully insert should be easier 
// just proof of concept  and opening for starting lib

use mysql;
use mysql::Value;
// mirror of our test db
#[derive(Debug)]
struct Books {
	bookid: i32,
	title: String,
	seriesid: i32,
	authorid: i32
}

fn main() {
    //specify server info
    let connection_url = "mysql://root@localhost:3306/test";
	// set up what i believe is a pool of connections to the sql server
	let pool = mysql::Pool::new(connection_url).unwrap();
	// our vector of what will be rows
	let mut list_of_books = Vec::new();
	//black magic that then iterates over our rows but In a special Row type that is useless 
	for row in pool.prep_exec(r"SELECT * FROM books;",()).unwrap() {

	 	// rust likes to wrap things so we take them out and  stuff it in a Vec of another useless type Value
	 	let rows: Vec<Value> = row.unwrap().unwrap();
	 	
	 	// we set up a bunch of var that will carry our info out of the for loop, there is most certainly a more idiomatic way to do this
	 	let mut bid: i32 = -1;
	 	let mut tl: String = "".to_string();
	 	let mut sid: i32 = -1;
	 	let mut aid: i32 = -1;
	 	
	 	// a loop for each of our values in our row and getting a "real" value out and storing it in its program counterpart
	 	for (index, row_value) in rows.iter().enumerate() {
		 			
		 			if index == 0 {	
		 				// turn it to a string to get it out of weird types and parse into useful values
		 				bid = row_value.as_sql(false).parse().unwrap();
		 			}
		 			else if index == 1 {
		 				tl = row_value.as_sql(false)
		 			}
		 			else if index == 2 {
		 				sid = row_value.as_sql(false).parse().unwrap();
		 			} 
		 			else if index == 3 {
		 				aid = row_value.as_sql(false).parse().unwrap();
		 			}

		 }
		//construct struct with our passed variables and push on to our list of books
		list_of_books.push(Books{bookid: bid, title: tl, seriesid: sid, authorid: aid});
	 	
	 } ;
	 //proof it was done
	 println!("this is end vector: {:?}", list_of_books );
	
}
