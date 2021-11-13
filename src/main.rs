fn main() {
    let mut args = std::env::args().skip(1);

    // Unwrap makes sure that we pass a 2nd param, if not it'll crash the program
    let key = args.next().unwrap();
    let value = args.next().unwrap();

    let mut db = Database::new().expect("Creating DB failed!");
    db.insert(key.to_uppercase(), value.clone());
    db.insert(key, value);
    db.flush().unwrap();
}

use std::collections::HashMap;
struct Database {
    row: HashMap<String, String>,
}

// impl means implmentations, so adding few implementations for the struct
impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();

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
            map.insert(key.to_owned(), value.to_owned());
        }

        Ok(Database { row: map })
    }

    // adding self as first arg make function->method
    fn insert(&mut self, key: String, value: String) {
        self.row.insert(key, value);
    }

    // here std::io::Result is same as Result<smthng,io::err>, with io error hardcoded
    fn flush(self) -> std::io::Result<()> {
        let mut contents = String::new();
        for pairs in &self.row {
            let kvpair = format!("{}\t{}\n",pairs.0,pairs.1);
            contents.push_str(&kvpair);
        }
            std::fs::write("kv.db", contents)
    }
    // Here in flush, we take DB by ownership. Technically we should try first borrowing it, then mutably borrowing it and then ownership
    // And borrwoing will work
    // But logically, if we called flush then there's no point of changing the DB, so we call it by ownership and hence DB will not extist after flush is called
    // Can be used in API devlopment
}
