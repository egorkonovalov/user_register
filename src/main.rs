use bcrypt::{hash, DEFAULT_COST};
use postgres::{Client, Error, NoTls};
use std::collections::HashMap;
use std::env;
use uuid::Uuid;

struct User {
    _id: Uuid,
    username: String,
    password: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let username: &String = &args[1];
    let password: &String = &args[2];
    let db_url: &String = &args[3];

    match sign_up(username, password, db_url) {
        Ok(()) => println!("Success!"),
        Err(error) => match error.code() {
            Some(value) => {
                if value.code() == "23505" {
                    println!("Username already exsits!")
                } else {
                    println!("SQL execution error!")
                }
            }
            None => println!("Unknown error!"),
        },
    };
}

fn sign_up(username: &String, password: &String, db_url: &String) -> Result<(), Error> {
    let mut client = Client::connect(db_url, NoTls)?;

    let hashed_password = hash_password(password);

    let mut user = HashMap::new();
    user.insert(username, hashed_password);

    for (key, value) in &user {
        let user = User {
            _id: Uuid::new_v4(),
            username: key.to_string(),
            password: value.to_string(),
        };

        client.execute(
            "INSERT INTO shop_user (username, password) VALUES ($1, $2)",
            &[&user.username, &user.password],
        )?;
    }

    Ok(())
}

fn hash_password(password: &String) -> String {
    let hashed_password = hash(password, DEFAULT_COST);
    match hashed_password {
        Ok(password) => return password,
        Err(error) => panic!("Problems {}", error),
    };
}
