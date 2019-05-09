#[macro_use]
extern crate diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
mod schema;
use std::path::Path;
extern crate chrono;
use chrono::NaiveDate;


#[derive(Debug)]
pub struct Work {
	struct_work_name: String,
	struct_creation_date: NaiveDate,
	struct_bequeathment: String,
	struct_source: String,
	struct_medium:String,
	struct_comment: String
}

#[derive(Debug)]
pub struct Pic<'a> {	
	struct_quality: bool,
	struct_fk_works_id: i32,
	struct_pic_path: &'a Path
}

#[derive(Debug)]
pub struct Location { 
	struct_location_date: NaiveDate,
	struct_location: String,
	struct_comment: String,
	struct_fk_works_id: i32 
}

#[derive(Queryable,  Debug)]
pub struct works{
        id: u32,
        work_name: String,
        creation_date: NaiveDate,
        location: String,
        bequeathment: String,
        source: String,
        medium: String,
        comment: String,
    }

impl Work {

	pub fn new( param_work_name: String,
				param_creation_date: NaiveDate,
				param_bequeathment: String,
				param_source: String,
				param_medium:String,
				param_comment: String,
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
			struct_work_name: size(param_work_name,500),
			struct_creation_date: param_creation_date,
			struct_bequeathment: size(param_bequeathment, 500),
			struct_source: size(param_source, 500),
			struct_medium: size(param_medium, 100),
			struct_comment: param_comment

		};
		use schema::works::dsl::*;
		//add new work to data base

		let _r = diesel::insert_into(works)
				.values(
					(
				        work_name.eq(&added_work.struct_work_name),
				        creation_date.eq(&added_work.struct_creation_date),
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
impl<'a> Pic<'a> {
	pub fn new( param_pic_path: &'a Path,
				param_quality: bool,
				param_fk_works_id: i32,
				conn: &MysqlConnection
				) -> Pic<'a>
		{
			// is existing id or panic	
			assert!(check_work_id(&conn, param_fk_works_id));
			
			//is good path or panic
			assert!(Path::exists(param_pic_path));
			assert!(Path::is_file(param_pic_path));
	 		//create pic struct
	 		let apic = Pic{
				struct_pic_path: param_pic_path,
				struct_quality: param_quality,
				struct_fk_works_id: param_fk_works_id
			};
			use schema::pics::dsl::*;
			//add new pic to db
			let _r = diesel::insert_into(pics)
				.values(
					(
				        pic_path.eq(&apic.struct_pic_path.to_str().unwrap()),
				        quality.eq(&apic.struct_quality),
				        fk_works_id.eq(&apic.struct_fk_works_id)
					)
				)
				.execute(conn);
	    	apic
		}
}
impl Location {
	fn new( param_location_date: NaiveDate, 
			param_location: String, 
			param_comment: String, 
			param_fk_works_id: i32,
			conn: &MysqlConnection) 
			-> Location 
		{
			//good work_id
			assert!(check_work_id(&conn, param_fk_works_id));
			
			let size = |x: String,y|{ 
				assert!( x.len() < y );
				x
			};
			
			let new_location = Location{
				struct_location_date: param_location_date,
				struct_location: size(param_location, 500),
				struct_comment: param_comment,
				struct_fk_works_id: param_fk_works_id
			};
			use schema::locations::dsl::*;
			let _r = diesel::insert_into(locations)
				.values(
					(
				        location_date.eq(&new_location.struct_location_date),
				        location.eq(&new_location.struct_location),
				        comment.eq(&new_location.struct_comment),
				        fk_works_id.eq(&new_location.struct_fk_works_id)
					)
				)
				.execute(conn);
    		new_location
		}
}
pub fn make_new_pool() -> MysqlConnection {
  	let connection_url = "mysql://root@localhost:3306/test";
	let conn = MysqlConnection::establish(connection_url).expect(&format!("Error connecting to {}", connection_url));
	conn
}
pub fn check_work_id(conn: &MysqlConnection, a_id: i32) -> bool {
	use schema::works::dsl::*;

	let a_id = a_id as u32;
	
	let return_value = works.find(a_id).execute(conn).unwrap();
	
	let return_value = return_value != 0;

	return_value
}











//------------------------------------Tests------------------------------------
#[cfg(test)]
mod test {
	use super::*;
	struct somevars<'a> {
		str499: String,
		str500: String,
		str100: String,
		str99: String,
		adate: NaiveDate,
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
			adate: NaiveDate::from_ymd(1996,3,13),
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
	#[test]
	#[ignore]
	fn new_location_test() {
		let x = vars();
		let should_work = Location::new(x.adate,x.location,x.comment,x.fk_works_id, &x.pool);
	}
	#[test]
	#[should_panic]
	#[ignore]
	fn bad_fk_id_location_test() {
		let x = vars();
		let wont_work = Location::new(x.adate,x.location,x.comment,x.bad_fk_id, &x.pool);
	}
	#[test]
	#[ignore]
	fn location_test() {
		let x = vars();
		let wont_work = Location::new(x.adate,x.str499,x.comment,x.fk_works_id, &x.pool);
	}
	#[test]
	#[should_panic]
	#[ignore]
	fn location_test_fail() {
		let x = vars();
		let wont_work = Location::new(x.adate,x.str500,x.comment,x.fk_works_id, &x.pool);
	}
	#[test]
	#[ignore]
	fn get_work_ids_test() {
		let x = vars();
		let should_work = get_work_ids(&x.pool);
	}
	#[test]
	#[ignore]
	fn make_new_pic_test() {
		let x = vars();
		let should_work = Pic::new(x.pic_path, x.quality, x.fk_works_id, &x.pool);
	}
	#[test]
	#[should_panic]
	#[ignore]
	fn bad_pic_path_test() {
		let x = vars();
		let wont_work = Pic::new(x.bad_pic_path, x.quality, x.fk_works_id, &x.pool);
	}
	#[test]
	#[should_panic]
	#[ignore]
	fn bad_fk_id_test() {
		let x = vars();
		let wont_work = Pic::new(x.pic_path, x.quality,x.bad_fk_id, &x.pool);
	}
	#[test]
	#[ignore]
	fn make_new_work_test() {
		let x = vars();
		let should_work = Work::new(x.work_name,x.adate,x.bequeathment,x.source,x.medium,x.comment,&x.pool);
	}
	
	#[test]
	#[ignore]
	fn work_name_test(){
		let x = vars();
		let should_work = Work::new(x.str499,x.adate,x.bequeathment,x.source,x.medium,x.comment,&x.pool);
	}
	#[test]
	#[should_panic]
	#[ignore]
	fn work_name_test_fail(){
		let x = vars();
		let wont_work = Work::new(x.str500,x.adate,x.bequeathment,x.source,x.medium,x.comment,&x.pool);
	}
	#[test]	
	#[ignore]
	fn bequeth_test() {
		let x = vars();
		let should_work = Work::new(x.work_name,x.adate,x.str499,x.source,x.medium,x.comment,&x.pool);
	}
	#[test]
	#[should_panic]
	#[ignore]
	fn bequeth_test_fail() {
		let x = vars();
		let wont_work = Work::new(x.work_name,x.adate,x.str500,x.source,x.medium,x.comment,&x.pool);
	}
	#[test]	
	#[ignore]
	fn source_test() {
		let x = vars();
		let should_work = Work::new(x.work_name,x.adate,x.bequeathment,x.str499,x.medium,x.comment,&x.pool);
	}
	#[test]
	#[should_panic]
	#[ignore]
	fn source_test_fail() {
		let x = vars();
		let wont_work = Work::new(x.work_name,x.adate,x.bequeathment,x.str500,x.medium,x.comment,&x.pool);
	}
	#[test]	
	#[ignore]
	fn medium_test() {
		let x = vars();
		let should_work = Work::new(x.work_name,x.adate,x.bequeathment,x.source,x.str99,x.comment,&x.pool);
	}
	#[test]
	#[should_panic]
	#[ignore]
	fn medium_test_fail() {
		let x = vars();
		let wont_work = Work::new(x.work_name,x.adate,x.bequeathment,x.source,x.str100,x.comment,&x.pool);
	}
}