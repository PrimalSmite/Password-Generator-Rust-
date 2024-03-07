use rand::Rng;
use std::io;

pub mod password{
    const SYMBOLS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
        
    let mut rng = rand::thread_rng();
    
    fn ask(){
        println!("Введите количество символов для пароля:");
        let mut count:i32;
        match io::stdin().read_line(&mut count){
            Ok() => {
                return count;
            },
            Err(e) => {
                println!("Ошибка - {}",e);
            }
        }

    }

    pub fn generate(count: i32, symbols: String){
        ask();  
    }
}
