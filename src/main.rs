use mysql;

fn main() {
    let connection_url = "mysql://root@localhost:3306/test";
	let pool = mysql::Pool::new(connection_url).unwrap();
	let var = pool.prep_exec(r"SELECT * FROM books;",()).unwrap();
	println!("{:?}",var );
}
