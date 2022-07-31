// Take a look at the generated `cornucopia.rs` file if you want to
// see what it looks like under the hood.
mod cornucopia;

use postgres::{Config, NoTls};

use crate::cornucopia::{
    queries::{
        module_1::insert_book,
        module_2::{
            author_name_by_id, author_name_starting_with, authors, books, select_translations,
            select_where_custom_type, AuthorNameStartingWithParams,
        },
    },
    types::public::SpongebobCharacter,
};

use cornucopia_sync::Params;

pub fn main() {
    // Connection pool configuration
    // Please look at the `postgres` crate for details
    let mut client = Config::new()
        .user("postgres")
        .password("postgres")
        .host("127.0.0.1")
        .port(5435)
        .dbname("postgres")
        .connect(NoTls)
        .unwrap();

    // Queries accept regular clients.
    // The `all` method returns all rows in a `Vec`
    println!("{:?}", authors().bind(&mut client).all().unwrap());

    {
        // Queries also accept transactions
        let mut transaction = client.transaction().unwrap();

        // Insert a book
        // Note that queries with a void return type (such as regular inserts)
        // don't need to call `all`, they are executed as soon as you `bind` them.
        insert_book()
            .bind(&mut transaction, &"The Great Gatsby")
            .unwrap();

        // You can use a map to transform rows ergonomically.
        let uppercase_books = books()
            .bind(&mut transaction)
            .map(|b| b.to_uppercase())
            .all()
            .unwrap();
        println!("{uppercase_books:?}");

        // Don't forget to `.commit()` the transaction when you're done!
        // Otherwise, it will be rolled back without further effect.
        transaction.commit().unwrap();
    }

    // Using `opt` returns an optional row (zero or one).
    // Any other number of rows will return an error.
    println!(
        "{:?}",
        author_name_by_id().bind(&mut client, &0).opt().unwrap()
    );

    // Using named structs as parameters and rows can be more convenient
    // and less error-prone, for example when a query has a lot of parameters.
    // This query doesn't benefit much, but is still shown for demonstration purposes.
    // ! Note: To use this feature you need to:
    // ! 1. Declare this custom type in your query files
    // !    (see `queries/module_2.sql` to see how this type was declared).
    // ! 2. Import the `Params` trait.
    println!(
        "{:?}",
        author_name_starting_with()
            .params(
                &mut client,
                &AuthorNameStartingWithParams { start_str: &"Jo" }
            )
            .all()
            .unwrap()
    );

    // Custom PostgreSQL types from your queries also work!
    // This includes domains, composites and enums.
    // They will be automatically generated by Cornucopia.
    // You can use them as bind parameters (as shown here)
    // or receive them in returned rows.
    println!(
        "{:?}",
        select_where_custom_type()
            .bind(&mut client, &SpongebobCharacter::Patrick)
            .one()
            .unwrap()
    );

    // Cornucopia also supports PostgreSQL arrays, which you
    // can use as bind parameters or in returned rows.
    println!(
        "{:?}",
        select_translations()
            .bind(&mut client)
            .map(|row| format!("{}: {:?}", row.title, row.translations))
            .all()
            .unwrap()
    );
}
