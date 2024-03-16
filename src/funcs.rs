//Модуль пароля
pub mod password{
    use crate::input::input::input_int;
    use rand::Rng;

    pub const SYMBOLS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#!";


    fn ask_len() -> u8 {
        println!("Введите количество символов в пароле:");
        let mut count = String::new();
        /*
        io::stdin().read_line(&mut _count).expect("error");
        let len_int: u8 = transform(&_count);
        */
        let len: u8 = input_int(&mut count);

        len
    }

    pub fn generate() -> String{
        let mut rng = rand::thread_rng();
       
        let lenght: u8 = ask_len(); 

        let password: String = (0..lenght)
            .map(|_| {
                let index = rng.gen_range(0..SYMBOLS.len());
                SYMBOLS[index] as char
            })
            .collect();
        println!("{:?}", password);

        password
    }
}



/*Модуль базы данных
pub mod sql {
    extern crate rusqlite;

    use std::{fmt::Result, io};
    use rusqlite::{Connection, Result};

    pub fn connect() -> Result<()> {
        let con = Connection::open("Passwords.db");

        println!("db is opened!");
        Ok(())
    }

    pub fn save(connect: &Result<()>) {
        
        if connect == Ok(()) {
            println!("ghsd");
        } else if connect == Err(()) {

        }
    }
}
*/
