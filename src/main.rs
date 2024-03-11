mod funcs;
use std::io;
pub use crate::funcs::password;

/*
let mut action = String::new();
fn menu() -> u8{
    println!("Выберите действие:
             (1) Создать пароль
             (2) Сохранить пароль
             (0) Выход");

    //let mut action = String::new();
    io::stdin().read_line(&mut action).expect("error");
    let _act_int: u8 = password::transform(&action);
    _act_int
}
*/
fn main(){
    let act_str = String::new();
    let act: u8;
    fn menu(string: &String) -> &u8{
        println!("Выберите действие:
        (1) Создать пароль
        (2) Сохранить пароль
        (0) Выход");

        io::stdin().read_line(&mut string).expect("Не удалось ввести данные");

    }

    while menu_act != 0 {
       if menu_act == 1 {
           password::generate(password::SYMBOLS);
           println!("Хотите сохранить пароль?
                    (1) Да
                    (2) Нет");
           let mut save = String::new();
           io::stdin().read_line(&mut save).expect("Не удалось ввести данные");
           let save_int:u8 = password::transform(&save);
           
           break; 
       }
    }
}
