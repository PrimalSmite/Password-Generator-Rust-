mod funcs;
mod input;

use crate::funcs::password;
use crate::input::input::input_int;
 
pub use std::io;

mod menu{
    //use std::io;
    use crate::input::input::input_int;

    pub fn menu() -> u8{
        println!("Выберите действие:\n(1) Создать пароль\n(2) Сохранить пароль\n(0) Выход");

        //Ввод числа
        let mut menu_act = String::new();
        let action: u8 = input_int(&mut menu_act);

        action
    }
}


fn main(){
    let act = menu::menu();

    //Основной цикл
    while act != 0 {

        //Создание пароля
       if act == 1 {
           password::generate();
           println!("Хотите сохранить пароль?\n(1) Да\n(2) Нет");

           //Ввод числа
           let mut save_string = String::new();
           let save:u8 = input_int(&mut save_string);
           
           //Сохранение пароля
            if save == 1{
                
            } else if save == 2 {
                //sql::save(&sql::connect()); 
            } else {

            }
           
           break; 
       }
    }
}
