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

