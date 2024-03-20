pub mod db{
    extern crate rusqlite;

    use rusqlite::{Connection, Result};

    pub fn save(mut _name: &String, mut _login: &String, mut _password: &String) -> Result<()>{
       
        //Путь до базы данных
        let path = "./Passwords.db";
        let db = Connection::open(path)?;
        
        //Создание таблицы, если её ещё нет
        db.execute(
            "CREATE TABLE IF NOT EXISTS passwords ( 
                id          INTEGER PRIMARY KEY,
                name        TEXT NOT NULL,
                login       TEXT NOT NULL,
                password    TEXT NOT NULL
            )",
            (), //Пустое значение параметров
        )?;
        
        //Ввод значений в таблицу
        db.execute(
            "INSERT INTO passwords (id, login, password) VALUES (?1, ?2, ?3)",
            (_name, _login, _password),
        )?;
        
        Ok(())
    }

}
