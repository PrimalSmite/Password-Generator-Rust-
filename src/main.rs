pub use crate::funcs::{password};
pub use crate::input::*;
mod funcs; 
mod input; 
pub use std::io;

mod menu{
    use std::io;
    use crate::password;

    pub fn menu() -> u8{
        println!("Выберите действие:
        (1) Создать пароль
        (2) Сохранить пароль
        (0) Выход");


        //Ввод числа
        let mut menu_act = String::new();
        io::stdin().read_line(&mut menu_act).expect("error");
        let action:u8 = password::transform(&menu_act);
        action
    }
}


fn main(){
    let act = menu::menu();

    //Основной цикл
    while act != 0 {

        //Создание пароля
       if act == 1 {
           password::generate(password::SYMBOLS);
           println!("Хотите сохранить пароль?
                    (1) Да
                    (2) Нет");

           //Ввод числа
           let mut save_string = String::new();
           io::stdin().read_line(&mut save_string).expect("Не удалось ввести данные");
           let save:u8 = password::transform(&save_string);
           
           //Сохранение пароля
            if save == 1{
                
            } else if save == 2 {
                sql::save(&sql::connect()); 
            } else {

            }
           
           break; 
       }
    }
}
