//Модуль пароля
pub mod password{
    use crate::input::input::int_input;
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
        let len: u8 = int_input(&mut count);

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
