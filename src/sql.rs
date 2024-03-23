pub mod db{
    extern crate rusqlite;

    use rusqlite::{Connection, Result};

    fn connect() -> Result<Connection, Error>{
        let conn = Connection::open("Passwords.db");
        conn
    }

    //Сохранение
    pub fn save(mut _name: &str, mut _login: &str, mut _password: &str, conn: &Connection) -> Result<()>{
       
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
            )", ()
            ){
            Err(err) => {
                println!("{} create failed", err);
                Err(err)
            },
            Ok(created) => {
                println!("{} table created", created);
                
                match db.execute("INSERT INTO passwords (name, login, password) VALUES (:name, :login, :password)", &[(":name", _name,
                                                                                                                       ":login", _login,
                                                                                                                       ":password", _password)]){
                    Err(err) => {
                        println!("{} insert failed", err);
                        Err(err)
                    }
                    Ok(inserted) => {
                        println!("{} inserted", inserted);


                        Ok(())
                    }
                }
            }
            
        } 

    }
}
