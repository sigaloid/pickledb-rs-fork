//! An example of how to use lists in PickleDB. It includes:
//! * Creating a new DB
//! * Loading an existing DB from a file
//! * Creating and removing lists
//! * Adding and removing items of different type to lists
//! * Retrieving list items

#[cfg(feature = "nano")]
use nanoserde::{DeBin, SerBin};
#[cfg(any(feature = "bincode", feature = "nano"))]
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
#[cfg(feature = "bincode")]
use serde::{Deserialize, Serialize};
#[cfg(any(feature = "bincode", feature = "nano"))]
use std::fmt::{self, Display, Formatter};

/// Define an example struct which represents a rectangle.
/// Next we'll show how to use it in lists.
#[cfg(feature = "bincode")]
#[derive(Serialize, Deserialize)]
struct Rectangle {
    width: i32,
    length: i32,
}

#[cfg(feature = "nano")]
#[derive(SerBin, DeBin)]
struct Rectangle {
    width: i32,
    length: i32,
}

#[cfg(any(feature = "bincode", feature = "nano"))]
impl Display for Rectangle {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Rectangle: length={}, width={}", self.length, self.width)
    }
}

/// Create a new DB and add one key-value pair to it
#[cfg(any(feature = "bincode", feature = "nano"))]
fn create_db(db_name: &str) {
    let mut new_db = PickleDb::new(
        db_name,
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Bin,
    );

    new_db.set("key1", &100).unwrap();
}

fn main() {
    #[cfg(any(feature = "bincode", feature = "nano"))]
    {
        // create a new DB
        create_db("example.db");

        // load the DB
        let mut db = PickleDb::load(
            "example.db",
            PickleDbDumpPolicy::AutoDump,
            SerializationMethod::Bin,
        )
        .unwrap();

        // print the existing value in key1
        println!("The value of key1 is: {}", db.get::<i32>("key1").unwrap());

        // create a new list
        db.lcreate("list1")
            .unwrap()
            // add an integer item to the list
            .ladd(&200_i32)
            // add an floating point item to the list
            .ladd(&2.1_f64)
            // add a string to the list
            .ladd(&String::from("my list"))
            // add multiple values to the list: add 3 rectangles
            .lextend(&[
                Rectangle {
                    width: 2,
                    length: 4,
                },
                Rectangle {
                    width: 10,
                    length: 22,
                },
                Rectangle {
                    width: 1,
                    length: 22,
                },
            ]);

        // print the list length
        println!("list1 length is: {}", db.llen("list1"));

        // print the item in each position of the list
        println!("list1[0] = {}", db.lget::<i32>("list1", 0).unwrap());
        println!("list1[1] = {}", db.lget::<f64>("list1", 1).unwrap());
        println!("list1[2] = {}", db.lget::<String>("list1", 2).unwrap());
        println!("list1[4] = {}", db.lget::<Rectangle>("list1", 4).unwrap());
        println!("list1[6] = {}", db.lget::<Rectangle>("list1", 5).unwrap());
        println!("list1[7] = {}", db.lget::<Rectangle>("list1", 6).unwrap());

        // remove an item in the list
        db.lpop::<i32>("list1", 0);

        // print the new first item of the list
        println!("The new list1[0] = {}", db.lget::<f64>("list1", 0).unwrap());

        // remove the entire list
        db.lrem_list("list1").unwrap();

        // was list1 removed?
        println!(
            "list1 was removed. Is it still in the db? {}",
            db.lexists("list1")
        );

        // create a new list
        db.lcreate("list2").unwrap().lextend(&[1, 2, 3, 4]);

        // iterate over the items in list2
        for item_iter in db.liter("list2") {
            println!("Current item is: {}", item_iter.get_item::<i32>().unwrap());
        }
    }
}
