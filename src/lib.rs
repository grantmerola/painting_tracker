use mysql::Pool;
use mysql::params;
use mysql::Value;
use std::path::Path;

#[derive(Debug)]
pub struct Work {
	work_name: String,
	creation_date: Value,
	location: String,
	bequeathment: String,
	source: String,
	medium:String,
	comment: String
}

#[derive(Debug)]
pub struct Pic<'a> {
	
	quality: bool,
	fk_works_id: i32,
	pic_path: &'a Path
}

#[derive(Debug)]
pub struct Locations { 
	location_date: Value,
	location: String,
	comment: String,
	fk_works_id: i32 
}

impl Work {
	pub fn new( work_name: String,
				creation_date: Value,
				location: String,
				bequeathment: String,
				source: String,
				medium:String,
				comment: String,
				pool: &Pool
				) -> Work 
		{

		let size = |x: String,y|{ 
				if x.len() < y {
					return x
				}
				else {
					panic!("to long");
				}
			};
		let addedWork = Work {
			work_name: size(work_name,500),
			creation_date: creation_date,
			location: location,
			bequeathment: size(bequeathment, 500),
			source: size(source, 500),
			medium: size(medium, 100),
			comment: comment
		};

		for mut stmt in pool.prepare(r"INSERT INTO works
			                               (work_name,creation_date,location,bequeathment,source,medium,comment)
			                           VALUES
			                               (:work_name, :creation_date, :location, :bequeathment, :source, :medium, :comment)").into_iter() {
	       
            stmt.execute(params!{
                "work_name" => &x.work_name,
                "creation_date" => &x.creation_date,
                "location" => &x.location,
                "bequeathment" => &x.bequeathment,
                "source" => &x.source,
                "medium" => &x.medium,
                "comment" => &x.comment
            }).unwrap();    
    	}

	addedWork
	}
}

fn make_new_pool() -> Pool {
	let connection_url = "mysql://root@localhost:3306/test";
	
	let pool = mysql::Pool::new(connection_url);

	let pool = match pool {
		Ok(poolcon) => poolcon,
		Err(error) => {
			panic!("panicked while trying to connect to server: {:?}", error)
		},
	};
	pool
}











//------------------------------------Tests------------------------------------
#[cfg(test)]
mod test {
	use super::*;
	#[derive(Debug)]
	struct somevars {
		str499: String,
		str500: String,
		str100: String,
		str99: String,
		adate: Value,
		work_name: String,
		bequeathment: String,
		source: String,
		medium: String,
		location: String,
		comment: String,
		pool: Pool
	}
	fn vars() -> somevars {
		somevars{
			str499: String::from("khgkhgkgkgkhgkhgkhgkhgkgkhgkgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkgkgkhgkhgkhgkhgkgkhgkhgkhgkgkhgkhkhlghvfhfghjhdgfsghjkfgfdghjkhjghfdgrhfjgkhjlkljhgfdghjkldfghjkl;jhgfhjkljhgfdjgkhgkfjtgkhlukgyfjchkjghjkugjuyghjkilouyghjkiuyfghuiytghjuytrfghuyt6ryfhgjkiurtydfghjliuytfghjuytfhgbkuyutfgcvbhkygfhgkjuygjhfnbhmkyfghkugjhkuygtfhgkuytfhgkuytfgghjgytrdgfhjytfhgjkuiygtfhjkgfhghjyj"),	
			str500: String::from("dkhgkhgkgkgkhgkhgkhgkhgkgkhgkgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkgkgkhgkhgkhgkhgkgkhgkhgkhgkgkhgkhkhlghvfhfghjhdgfsghjkfgfdghjkhjghfdgrhfjgkhjlkljhgfdghjkldfghjkl;jhgfhjkljhgfdjgkhgkfjtgkhlukgyfjchkjghjkugjuyghjkilouyghjkiuyfghuiytghjuytrfghuyt6ryfhgjkiurtydfghjliuytfghjuytfhgbkuyutfgcvbhkygfhgkjuygjhfnbhmkyfghkugjhkuygtfhgkuytfhgkuytfgghjgytrdgfhjytfhgjkuiygtfhjkgfhghjyj"),
			str100: String::from("sdkhgkhgkgkgkhgkhgkhgkhgkgkhgkgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkhgkhgkhgkgkhg"),
			str99: String::from("dkhgkhgkgkgkhgkhgkhgkhgkgkhgkgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkhgkhgkhgkgkhg"),
			adate: Value::Date(1994,0,0,0,0,0,0),
			work_name: String::from("atitle"),
			bequeathment: String::from("burned"),
			source: String::from("Montana"),
			medium: String::from("oil"),
			location: String::from("portland"),
			comment: String::from("what a work"),
			pool: make_new_pool()
		}
	}
	

	#[test]
	fn make_new() {
		let x = vars();
		let should_work = Work::new(x.work_name,x.adate,x.location,x.bequeathment,x.source,x.medium,x.comment,&x.pool);
	}
	
	#[test]
	#[ignore]
	fn work_name_test(){
		let x = vars();
		let should_work = Work::new(x.str499,x.adate,x.location,x.bequeathment,x.source,x.medium,x.comment,&x.pool);
	}
	#[test]
	#[should_panic]
	#[ignore]
	fn work_name_test_fail(){
		let x = vars();
		let should_work = Work::new(x.str500,x.adate,x.location,x.bequeathment,x.source,x.medium,x.comment,&x.pool);
	}
	#[test]	
	#[ignore]
	fn bequeth_test() {
		let x = vars();
		let should_work = Work::new(x.work_name,x.adate,x.location,x.str499,x.source,x.medium,x.comment,&x.pool);
	}
	#[test]
	#[should_panic]
	#[ignore]
	fn bequeth_test_fail() {
		let x = vars();
		let should_work = Work::new(x.work_name,x.adate,x.location,x.str500,x.source,x.medium,x.comment,&x.pool);
	}
	#[test]	
	#[ignore]
	fn source_test() {
		let x = vars();
		let should_work = Work::new(x.work_name,x.adate,x.location,x.bequeathment,x.str499,x.medium,x.comment,&x.pool);
	}
	#[test]
	#[should_panic]
	#[ignore]
	fn source_test_fail() {
		let x = vars();
		let should_work = Work::new(x.work_name,x.adate,x.location,x.bequeathment,x.str500,x.medium,x.comment,&x.pool);
	}
	#[test]	
	#[ignore]
	fn medium_test() {
		let x = vars();
		let should_work = Work::new(x.work_name,x.adate,x.location,x.bequeathment,x.source,x.str99,x.comment,&x.pool);
	}
	#[test]
	#[should_panic]
	#[ignore]
	fn medium_test_fail() {
		let x = vars();
		let should_work = Work::new(x.work_name,x.adate,x.location,x.bequeathment,x.source,x.str100,x.comment,&x.pool);
	}
}