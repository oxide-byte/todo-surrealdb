use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub(crate) async fn connect_db() -> Surreal<Client> {

    // Connect to the server
    let db = Surreal::new::<Ws>("localhost:8000")
        .await.expect("Cannot connect Network");

    // Select a specific namespace / database
    db.use_ns("test").use_db("test")
        .await.expect("Cannot connect to Namespace");

    // Login User
    db.signin(Root {
        username: "root",
        password: "root",
    }).await.expect("Cannot connect User");

    db
}