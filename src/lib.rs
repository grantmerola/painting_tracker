#[macro_use]
extern crate diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
mod schema;
use diesel::sql_types::Date;
use std::path::Path;
extern crate chrono;
use chrono::NaiveDate;

// #[derive(Debug)]
pub struct Work {
	struct_work_name: String,
	struct_creation_date: Date,
	struct_location: String,
	struct_bequeathment: String,
	struct_source: String,
	struct_medium:String,
	struct_comment: String
}

// #[derive(Debug)]
// pub struct Pic<'a> {	
// 	quality: bool,
// 	fk_works_id: i32,
// 	pic_path: &'a Path
// }

// #[derive(Debug)]
// pub struct Location { 
// 	location_date: MysqlType,
// 	location: String,
// 	comment: String,
// 	fk_works_id: i32 
// }

impl Work {
	pub fn new( struct_work_name: String,
				struct_creation_date: Date,
				struct_location: String,
				struct_bequeathment: String,
				struct_source: String,
				struct_medium:String,
				struct_comment: String,
				conn: &MysqlConnection
				) -> Work 
		{
		
		
		//closure that panics if string is bigger than given size
		let size = |x: String,y|{ 
				assert!( x.len() < y );
				x
			};
		//create work struct
		let added_work = Work {
			struct_work_name: size(struct_work_name,500),
			struct_creation_date: struct_creation_date,
			struct_location: struct_location,
			struct_bequeathment: size(struct_bequeathment, 500),
			struct_source: size(struct_source, 500),
			struct_medium: size(struct_medium, 100),
			struct_comment: struct_comment
		};
		use schema::works::dsl::*;
		//add new work to data base
		println!("{:?}",&added_work.struct_creation_date );
		let r = diesel::insert_into(works)
				.values(
					(
				        work_name.eq(&added_work.struct_work_name),
				        // creation_date.eq(&added_work.struct_creation_date.FromSql()),
				        location.eq(&added_work.struct_location),
				        bequeathment.eq(&added_work.struct_bequeathment),
				        source.eq(&added_work.struct_source),
				        medium.eq(&added_work.struct_medium),
				        comment.eq(&added_work.struct_comment)
					)
				)
				.execute(conn);

	added_work
	}
}
// impl<'a> Pic<'a> {
// 	pub fn new( pic_path: &'a Path,
// 				quality: bool,
// 				fk_works_id: i32,
// 				pool: &Pool
// 				) -> Pic<'a>
// 		{
// 			// is existing id or panic	
// 			assert!(get_work_ids(&pool).contains(&fk_works_id));
			
// 			//is good path or panic
// 			assert!(Path::exists(pic_path));
// 			assert!(Path::is_file(pic_path));
// 	 		//create pic struct
// 	 		let apic = Pic{
// 				pic_path: pic_path,
// 				quality: quality,
// 				fk_works_id: fk_works_id
// 			};
// 			//add new pic to db
// 			for mut stmt in pool.prepare(r"INSERT INTO pics
// 				                               (pic_path,quality,fk_works_id)
// 				                           VALUES
// 				                               (:pic_path,:quality,:fk_works_id)").into_iter() {
		       
// 	            stmt.execute(params!{
// 	                "pic_path" => &apic.pic_path.to_str().unwrap(),
// 	                "quality" => &apic.quality,
// 	                "fk_works_id" => &apic.fk_works_id
// 	            }).unwrap(); 
// 	    	}
// 	    	apic
// 		}
// }
// impl Location {
// 	fn new( location_date: Value, 
// 			location: String, 
// 			comment: String, 
// 			fk_works_id: i32,
// 			pool: &Pool) 
// 			-> Location 
// 		{
// 			//good work_id
// 			assert!(get_work_ids(&pool).contains(&fk_works_id));
			
// 			let size = |x: String,y|{ 
// 				assert!( x.len() < y );
// 				x
// 			};
			
// 			let new_location = Location{
// 				location_date: location_date,
// 				location: size(location, 500),
// 				comment: comment,
// 				fk_works_id: fk_works_id
// 			};
// 			for mut stmt in pool.prepare(r"INSERT INTO locations
// 	                               (location_date,location,comment,fk_works_id)
// 	                           VALUES
// 	                               (:location_date,:location,:comment,:fk_works_id)").into_iter() {
	       
// 	            stmt.execute(params!{
// 	                "location_date" => &new_location.location_date,
// 	                "location" => &new_location.location,
// 	                "comment" => &new_location.comment,
// 	                "fk_works_id" => &new_location.fk_works_id
// 	            }).unwrap();    
//     		}
//     		new_location
// 		}
// }
pub fn make_new_pool() -> MysqlConnection {
  	let connection_url = "mysql://root@localhost:3306/test";
	let conn = MysqlConnection::establish(connection_url).expect(&format!("Error connecting to {}", connection_url));
	conn
}
// pub fn get_work_ids(pool: &Pool) -> Vec<i32> {
	
// 	let mut ids: Vec<i32> = Vec::new();
	
// 	for row in pool.prep_exec(r"SELECT id FROM works;",()).unwrap() {
// 	 						//get a vec out of a row
// 	 	let new_i32: i32 =   row.unwrap().unwrap()
// 	 							//get first and only column
// 	 							.get(0).unwrap()
// 	 							//get string make in to i32
// 	 							.as_sql(false).parse().unwrap();
// 	 	ids.push(new_i32); 	
// 	 };
// 	 ids
// }











// //------------------------------------Tests------------------------------------
#[cfg(test)]
mod test {
	use super::*;
	struct somevars<'a> {
		str499: String,
		str500: String,
		str100: String,
		str99: String,
		adate: Date,
		work_name: String,
		bequeathment: String,
		source: String,
		medium: String,
		location: String,
		comment: String,
		pic_path: &'a Path,
		quality: bool,
		fk_works_id: i32,
		bad_fk_id: i32,
		bad_pic_path: &'a Path,
		pool: MysqlConnection
	}
	fn vars() -> somevars<'static> {
		somevars{
			str499: String::from("khgkhgkgkgkhgkhgkhgkhgkgkhgkgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkgkgkhgkhgkhgkhgkgkhgkhgkhgkgkhgkhkhlghvfhfghjhdgfsghjkfgfdghjkhjghfdgrhfjgkhjlkljhgfdghjkldfghjkl;jhgfhjkljhgfdjgkhgkfjtgkhlukgyfjchkjghjkugjuyghjkilouyghjkiuyfghuiytghjuytrfghuyt6ryfhgjkiurtydfghjliuytfghjuytfhgbkuyutfgcvbhkygfhgkjuygjhfnbhmkyfghkugjhkuygtfhgkuytfhgkuytfgghjgytrdgfhjytfhgjkuiygtfhjkgfhghjyj"),	
			str500: String::from("dkhgkhgkgkgkhgkhgkhgkhgkgkhgkgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkgkgkhgkhgkhgkhgkgkhgkhgkhgkgkhgkhkhlghvfhfghjhdgfsghjkfgfdghjkhjghfdgrhfjgkhjlkljhgfdghjkldfghjkl;jhgfhjkljhgfdjgkhgkfjtgkhlukgyfjchkjghjkugjuyghjkilouyghjkiuyfghuiytghjuytrfghuyt6ryfhgjkiurtydfghjliuytfghjuytfhgbkuyutfgcvbhkygfhgkjuygjhfnbhmkyfghkugjhkuygtfhgkuytfhgkuytfgghjgytrdgfhjytfhgjkuiygtfhjkgfhghjyj"),
			str100: String::from("sdkhgkhgkgkgkhgkhgkhgkhgkgkhgkgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkhgkhgkhgkgkhg"),
			str99: String::from("dkhgkhgkgkgkhgkhgkhgkhgkgkhgkgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkhgkhgkhgkgkhg"),
			adate: Date= NaiveDate::from_ymd(1996,3,13),
			work_name: String::from("atitle"),
			bequeathment: String::from("burned"),
			source: String::from("Montana"),
			medium: String::from("oil"),
			location: String::from("portland"),
			comment: String::from("what a work"),
			pic_path: Path::new("/Users/gm/Downloads/colorinanything.jpg"),
			quality: true,
			fk_works_id: 1,
			bad_fk_id: 500000,
			bad_pic_path: Path::new("/Users/gm/Downloads/kurchunk.txt"),
			pool: make_new_pool()
		}
	}
// 	#[test]
// 	#[ignore]
// 	fn new_location_test() {
// 		let x = vars();
// 		let should_work = Location::new(x.adate,x.location,x.comment,x.fk_works_id, &x.pool);
// 	}
// 	#[test]
// 	#[should_panic]
// 	#[ignore]
// 	fn bad_fk_id_location_test() {
// 		let x = vars();
// 		let wont_work = Location::new(x.adate,x.location,x.comment,x.bad_fk_id, &x.pool);
// 	}
// 	#[test]
// 	#[ignore]
// 	fn location_test() {
// 		let x = vars();
// 		let wont_work = Location::new(x.adate,x.str499,x.comment,x.fk_works_id, &x.pool);
// 	}
// 	#[test]
// 	#[should_panic]
// 	#[ignore]
// 	fn location_test_fail() {
// 		let x = vars();
// 		let wont_work = Location::new(x.adate,x.str500,x.comment,x.fk_works_id, &x.pool);
// 	}
// 	#[test]
// 	#[ignore]
// 	fn get_work_ids_test() {
// 		let x = vars();
// 		let should_work = get_work_ids(&x.pool);
// 	}
// 	#[test]
// 	#[ignore]
// 	fn make_new_pic_test() {
// 		let x = vars();
// 		let should_work = Pic::new(x.pic_path, x.quality, x.fk_works_id, &x.pool);
// 	}
// 	#[test]
// 	#[should_panic]
// 	#[ignore]
// 	fn bad_pic_path_test() {
// 		let x = vars();
// 		let wont_work = Pic::new(x.bad_pic_path, x.quality, x.fk_works_id, &x.pool);
// 	}
// 	#[test]
// 	#[should_panic]
// 	#[ignore]
// 	fn bad_fk_id_test() {
// 		let x = vars();
// 		let wont_work = Pic::new(x.pic_path, x.quality,x.bad_fk_id, &x.pool);
// 	}
	#[test]
	#[ignore]
	fn make_new_work_test() {
		let x = vars();
		let should_work = Work::new(x.work_name,x.adate,x.location,x.bequeathment,x.source,x.medium,x.comment,&x.pool);
	}
	
// 	#[test]
// 	#[ignore]
// 	fn work_name_test(){
// 		let x = vars();
// 		let should_work = Work::new(x.str499,x.adate,x.location,x.bequeathment,x.source,x.medium,x.comment,&x.pool);
// 	}
// 	#[test]
// 	#[should_panic]
// 	#[ignore]
// 	fn work_name_test_fail(){
// 		let x = vars();
// 		let wont_work = Work::new(x.str500,x.adate,x.location,x.bequeathment,x.source,x.medium,x.comment,&x.pool);
// 	}
// 	#[test]	
// 	#[ignore]
// 	fn bequeth_test() {
// 		let x = vars();
// 		let should_work = Work::new(x.work_name,x.adate,x.location,x.str499,x.source,x.medium,x.comment,&x.pool);
// 	}
// 	#[test]
// 	#[should_panic]
// 	#[ignore]
// 	fn bequeth_test_fail() {
// 		let x = vars();
// 		let wont_work = Work::new(x.work_name,x.adate,x.location,x.str500,x.source,x.medium,x.comment,&x.pool);
// 	}
// 	#[test]	
// 	#[ignore]
// 	fn source_test() {
// 		let x = vars();
// 		let should_work = Work::new(x.work_name,x.adate,x.location,x.bequeathment,x.str499,x.medium,x.comment,&x.pool);
// 	}
// 	#[test]
// 	#[should_panic]
// 	#[ignore]
// 	fn source_test_fail() {
// 		let x = vars();
// 		let wont_work = Work::new(x.work_name,x.adate,x.location,x.bequeathment,x.str500,x.medium,x.comment,&x.pool);
// 	}
// 	#[test]	
// 	#[ignore]
// 	fn medium_test() {
// 		let x = vars();
// 		let should_work = Work::new(x.work_name,x.adate,x.location,x.bequeathment,x.source,x.str99,x.comment,&x.pool);
// 	}
// 	#[test]
// 	#[should_panic]
// 	#[ignore]
// 	fn medium_test_fail() {
// 		let x = vars();
// 		let wont_work = Work::new(x.work_name,x.adate,x.location,x.bequeathment,x.source,x.str100,x.comment,&x.pool);
// 	}
}