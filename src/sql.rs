pub mod db{
    extern crate rusqlite;

    use rusqlite::{Connection, Result};

    //Сохранение
    pub fn save(mut _name: &String, mut _login: &String, mut _password: &String) -> Result<()>{
       
        //Путь до базы данных
        //let path = "Passwords.db";
        let db = Connection::open("Passwords.db")?;
        
        //Создание таблицы, если её ещё нет
        match db.execute(
            "CREATE TABLE IF NOT EXISTS passwords ( 
                id          INTEGER PRIMARY KEY,
                name        TEXT NOT NULL,
                login       TEXT NOT NULL,
                password    TEXT NOT NULL
            )", ()){
            Ok(crated) =>{
                println!("{} tables created ", crated);

            },
            Err(err) => {
                println!("{} create failed", err);
            }
        }
        
        //Ввод значений в таблицу
        match db.execute("INSERT INTO passwords (id, login, password) VALUES (?1, ?2, ?3)",(_name, _login, _password)){
            Ok(inserted) => {
                println!("{} bububaba", inserted);
                Ok(())
            },
            Err(err) => {
                println!("{} ne bububaba", err);
                Err(err)
            }
        }
        
        //Ok(())
    }
}
