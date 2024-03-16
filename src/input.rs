pub mod input{
    use std::io; 

    //Функция создает переменную из типа String в тип u8
    pub fn transform(string: &String) -> u8 {
        let number:u8 = string.trim().parse().expect("Не удалось распознать число. Пожалуйста, убедитесь, что вы ввели число от 0 до 255");

        number
    } 
    
    //Функция вводит данные с клавиатуры и записывает в переменню типа String
    fn stdinput(string: String) -> &String{
        //Ввод значения с клавитуры в переменную String
        io::stdin().read_line(&mut string).expect("Не удалось ввести значение");

        &string
    }

    //Функция вводит данные с клавиатуры и записывает их в тип u8
    pub fn input_int(string: &String) -> u8{
        string = stdinput(string);
        let number: u8 = transform(string);
        
        //Возвращает number:u8
        number
    }
}
