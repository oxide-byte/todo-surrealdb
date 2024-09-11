use chrono::{DateTime, Utc};
use leptos::logging;
use serde::Deserialize;
use surrealdb::sql::{Strand, Thing};
use crate::model::{Error, Todo};
use crate::repository::db_connection::connect_db;

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
    #[allow(dead_code)]
    title:Strand,
    #[allow(dead_code)]
    description: Strand,
    #[allow(dead_code)]
    created: Strand
}

pub async fn get_all() -> Result<Vec<Todo>, Error> {
    logging::log!("[REPOSITORY][get_all]");
    let client = connect_db().await;

    let records: Vec<Record> = client.select("todo").await?;

    let todos = records.iter().map(|x| Todo {
        id: x.id.to_string(),
        title: x.title.clone().as_string(),
        description: x.description.clone().as_string(),
        created: x.created.clone().as_string().parse::<DateTime<Utc>>().unwrap()
    }).collect::<Vec<Todo>>();

    Ok(todos)
}

pub async fn insert_todo(todo: Todo) -> Result<(), Error> {
    logging::log!("[REPOSITORY][insert_todo]");
    let client = connect_db().await;

    let _created: Vec<Record> = client
        .create("todo")
        .content(todo)
        .await?;

    Ok(())
}

pub async fn update_todo(todo: Todo) -> Result<(), Error> {
    logging::log!("[REPOSITORY][update_todo]");
    let client = connect_db().await;

    let _created: Vec<Record> = client.update("todo")
        .content(todo)
        .await?;

    Ok(())
}

pub async fn delete_todo(id: &str) -> Result<(), Error> {
    logging::log!("[REPOSITORY][delete_todo] {}",id);
    let client = connect_db().await;

    // NOT WORKING
    let result1: Option<Record> = client
        .delete(("todo", id.to_string()))
        .await?;

    logging::log!("RS1: {:?}", result1);

    // NOT WORKING
    let result2 = client
        .query("DELETE $id")
        .bind(("id", id.to_string()))
        .await;

    logging::log!("RS2: {:?}", result2);

    // WORKING
    let result3 = client
        .query(format!("DELETE {}", id))
        .await;

    logging::log!("RS3: {:?}", result3);

    Ok(())
}