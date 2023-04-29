use std::collections::HashMap;

use crate::table::Table;

struct DatabaseMetaData {
    page_size: u16,
    pages_number: u32,
    changes_counter: u32,
    locked: bool,
}

struct Database {
    name: String,

    tables: HashMap<String, Table>,
}

impl Database {
    fn new() {}

    fn has_table() {}

    fn get_table() {}
}

//create table test(
//    id integer primary key,
//    username varchar(255),
//    email varchar(255),
//    password varchar(255),
//    age integer,
//    phone_number integer
//)

//insert into test (username ,email ,password ,age ,phone_number) values ('ilyes', 'ilyes@gmail.com', 'password', 22, 05555555);
