use crate::db::establish_connection;
use crate::models::*;
use diesel::prelude::*;

pub fn show_users() {
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let results = users
        .limit(5)
        .load::<User>(connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}", user.token);
        println!("-----------\n");
        println!("{}", user.balance);
    }
}
