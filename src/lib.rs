//use mysql;


	
	#[derive(Debug)]
	pub enum Date {
		YYYY_MM_DD(i32, i32, i32),
		YYYY_MM(i32, i32),
		YYYY(i32)		
	}
	
	#[derive(Debug)]
	pub struct Work {
		work_name: String,
		creation_date: Date,
		location: String,
		bequeathment: String,
		source: String,
		medium:String,
		comment: String
	}
	
	#[derive(Debug)]
	pub struct Pics {
    	pic_path: String,
    	quality: bool,
    	fk_works_id: i32
	}

	#[derive(Debug)]
	pub struct Locations { 
    	location_date: Date,
    	location: String,
    	comment: String,
    	fk_works_id: i32 
	}

	impl Work {
		pub fn new( work_name: String,
					creation_date: Date,
					location: String,
					bequeathment: String,
					source: String,
					medium:String,
					comment: String 
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
			Work {
				work_name: size(work_name,500),
				creation_date: creation_date,
				location: location,
				bequeathment: size(bequeathment, 500),
				source: size(source, 500),
				medium: size(medium, 100),
				comment: comment
			}
		}
	}

#[cfg(test)]
mod test {
	use super::*;
	#[derive(Debug)]
	struct somevars {
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
		comment: String
	}
	fn vars() -> somevars {
		somevars{
			str499: String::from("khgkhgkgkgkhgkhgkhgkhgkgkhgkgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkgkgkhgkhgkhgkhgkgkhgkhgkhgkgkhgkhkhlghvfhfghjhdgfsghjkfgfdghjkhjghfdgrhfjgkhjlkljhgfdghjkldfghjkl;jhgfhjkljhgfdjgkhgkfjtgkhlukgyfjchkjghjkugjuyghjkilouyghjkiuyfghuiytghjuytrfghuyt6ryfhgjkiurtydfghjliuytfghjuytfhgbkuyutfgcvbhkygfhgkjuygjhfnbhmkyfghkugjhkuygtfhgkuytfhgkuytfgghjgytrdgfhjytfhgjkuiygtfhjkgfhghjyj"),	
			str500: String::from("dkhgkhgkgkgkhgkhgkhgkhgkgkhgkgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkhgkgkgkhgkhgkhgkhgkgkhgkhgkhgkgkhgkhkhlghvfhfghjhdgfsghjkfgfdghjkhjghfdgrhfjgkhjlkljhgfdghjkldfghjkl;jhgfhjkljhgfdjgkhgkfjtgkhlukgyfjchkjghjkugjuyghjkilouyghjkiuyfghuiytghjuytrfghuyt6ryfhgjkiurtydfghjliuytfghjuytfhgbkuyutfgcvbhkygfhgkjuygjhfnbhmkyfghkugjhkuygtfhgkuytfhgkuytfgghjgytrdgfhjytfhgjkuiygtfhjkgfhghjyj"),
			str100: String::from("sdkhgkhgkgkgkhgkhgkhgkhgkgkhgkgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkhgkhgkhgkgkhg"),
			str99: String::from("dkhgkhgkgkgkhgkhgkhgkhgkgkhgkgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkhgkhgkhgkgkhgkhgkhgkhgkhgkhgkhgkhgkgkhg"),
			// use mysql::value::Date ?
			adate: Date::YYYY(1994),
			work_name: String::from("atitle"),
			bequeathment: String::from("burned"),
			source: String::from("Montana"),
			medium: String::from("oil"),
			location: String::from("portland"),
			comment: String::from("what a work")
		}
	}
	
	#[test]
	fn work_name_test(){
		let x = vars();
		let should_work = Work::new(x.str499,x.adate,x.location,x.bequeathment,x.source,x.medium,x.comment);
	}
	#[test]
	#[should_panic]
	fn work_name_test_fail(){
		let x = vars();
		let should_work = Work::new(x.str500,x.adate,x.location,x.bequeathment,x.source,x.medium,x.comment);
	}
	#[test]	
	fn bequeth_test() {
		let x = vars();
		let should_work = Work::new(x.work_name,x.adate,x.location,x.str499,x.source,x.medium,x.comment);
	}
	#[test]
	#[should_panic]
	fn bequeth_test_fail() {
		let x = vars();
		let should_work = Work::new(x.work_name,x.adate,x.location,x.str500,x.source,x.medium,x.comment);
	}
	#[test]	
	fn source_test() {
		let x = vars();
		let should_work = Work::new(x.work_name,x.adate,x.location,x.bequeathment,x.str499,x.medium,x.comment);
	}
	#[test]
	#[should_panic]
	fn source_test_fail() {
		let x = vars();
		let should_work = Work::new(x.work_name,x.adate,x.location,x.bequeathment,x.str500,x.medium,x.comment);
	}
	#[test]	
	fn medium_test() {
		let x = vars();
		let should_work = Work::new(x.work_name,x.adate,x.location,x.bequeathment,x.source,x.str99,x.comment);
	}
	#[test]
	#[should_panic]
	fn medium_test_fail() {
		let x = vars();
		let should_work = Work::new(x.work_name,x.adate,x.location,x.bequeathment,x.source,x.str100,x.comment);
	}
}