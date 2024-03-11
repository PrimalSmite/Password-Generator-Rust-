pub mod password{
    use std::io;
    use rand::Rng;

    pub const SYMBOLS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";

    pub fn transform(string: &str) -> u8 {
        let number:u8 = string.trim().parse().expect("Не удалось распознать число. Пожалуйста, убедитесь, что вы ввели число от 0 до 255");

        number
    } 

    pub fn ask_len() -> u8 {
        println!("Введите количество символов в пароле:");
        let mut _count = String::new();
        io::stdin().read_line(&mut _count).expect("error");
        let len_int: u8 = transform(&_count);

        len_int
    }

    pub fn generate(symbols: &[u8]) -> String{
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
