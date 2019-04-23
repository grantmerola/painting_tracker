// This is how to use the db at least select hopefully insert should be easier 
// just proof of concept  and opening for starting lib
#[macro_use]
extern crate diesel;
extern crate painting_tracker;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
//use painting_tracker::*;
mod schema;
use schema::books::dsl::*;
//use schema::books::*;
// mirror of our test db

#[derive(Queryable,  Debug)]
struct Book {
	bookid: i32,
	title: String,
	seriesid: Option<i32>,
	authorid: Option<i32>
}

fn main() {
    
    //specify server info
  	let connection_url = "mysql://root@localhost:3306/test";
	let conn = MysqlConnection::establish(connection_url).expect(&format!("Error connecting to {}", connection_url));
	
	let r: Vec<Book> = books.load::<Book>(&conn).unwrap();

	println!("{:?}",r );

	let tb = Book {
		bookid: 42,
		title: String::from("The Hitchhikers Guide to the Galaxy"),
		seriesid: Some(42),
		authorid: Some(43)
	};
	let d = diesel::insert_into(books)
					.values((
							Title.eq(tb.title),
							SeriesID.eq(tb.seriesid),
							AuthorID.eq(tb.authorid)
							));
					//.execute(&conn);
	println!("{:?}",d );
	
	println!("---------RAN---------");
}
