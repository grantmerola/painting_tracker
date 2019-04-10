use mysql;

mod accessdb {
	#[derive(Debug)]
	enum Date {
		YYYY_MM_DD(i32, i32, i32),
		YYYY_MM(i32, i32),
		YYYY(i32)		
	}
	
	#[derive(Debug)]
	struct Work {
		work_name: String,
		creation_date: Date,
		location: String,
		bequeathment: String,
		source: String,
		medium:String,
		comment: String
	}
	
	#[derive(Debug)]
	struct Pics {
    	pic_path: String,
    	quality: bool,
    	fk_works_id: i32
	}

	#[derive(Debug)]
	struct Locations { 
    	location_date: Date,
    	location: String,
    	comment: String,
    	fk_works_id: i32 
	}
}