fn main() {
    let mut args = std::env::args().skip(1);

    // Unwrap makes sure that we pass a 2nd param, if not it'll crash the program
    let key = args.next().unwrap();
    let value = args.next().unwrap();

    let mut db = Database::new().expect("Creating DB failed!");
    db.insert(key.to_uppercase(), value.clone());
    db.insert(key, value);
    match db.flush() {
        Ok(()) => print!("Done!"),
        Err(err) => print!("Oops! Some error happend: {}", err),
    };
}

use std::collections::HashMap;
struct Database {
    row: HashMap<String, String>,
    flush: bool,
}

// impl means implmentations, so adding few implementations for the struct
impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut row = HashMap::new();

        //  1) Read the kv.db file

        // let contents = match std::fs::read_to_string("kv.db") {
        //     Ok(c) => c,
        //     Err(e) => return Err(e),
        // };
        // Above can be re-written as
        let contents = std::fs::read_to_string("kv.db")?;

        // 2)  parse the string
        for line in contents.lines() {
            // 3)  populate our map
            let (key, value) = line.split_once('\t').expect("Corrupt database");
            row.insert(key.to_owned(), value.to_owned());
        }

        Ok(Database { row, flush: false })
    }

    // adding self as first arg make function->method
    fn insert(&mut self, key: String, value: String) {
        self.row.insert(key, value);
    }

    // here std::io::Result is same as Result<smthng,io::err>, with io error hardcoded
    fn flush(mut self) -> std::io::Result<()> {
        self.flush = true;
        do_flush(&self)
    }
    // Here in flush, we take DB by ownership. Technically we should try first borrowing it, then mutably borrowing it and then ownership
    // And borrwoing will work
    // But logically, if we called flush then there's no point of changing the DB, so we call it by ownership and hence DB will not extist after flush is called
    // Can be used in API devlopment
}

// This will by defualt always implement Drop on database, even if its not called manually
impl Drop for Database {
    fn drop(&mut self) {
        if !self.flush {
            let _ = do_flush(self);
        }
    }
}

fn do_flush(database: &Database) -> std::io::Result<()> {
    let mut contents = String::new();

    for (key, value) in &database.row {
        contents.push_str(key);
        contents.push('\t'); // Pushes a single char
        contents.push_str(value);
        contents.push('\n');
    }

    std::fs::write("kv.db", contents)
}
