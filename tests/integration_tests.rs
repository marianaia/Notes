/// This module contains integration tests for a task management API.
use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use lazy_static::lazy_static;
use reqwest::Client;
use std::collections::HashMap;

lazy_static! {
    static ref URL: String = {
        dotenv().ok();
        dotenv!("URL").to_string()
    };
}

#[tokio::test]
async fn test_create_task() {
    let endpoint_url = format!("{}/task", *URL);
    let mut map = HashMap::new();
    map.insert("title", "Sample Task");
    map.insert("task", "This is a sample task");

    let client = Client::new();
    let res = client
        .post(&endpoint_url)
        .json(&map)
        .send()
        .await
        .expect("Failed to create task");

    assert!(res.status().is_success());
}

#[tokio::test]
async fn test_get_one_task() {
    let client = Client::new();
    let endpoint_url = format!("{}/task/2", *URL);

    let res = client
        .get(&endpoint_url)
        .send()
        .await
        .expect("Failed to get task");

    assert!(res.status().is_success());
}

#[tokio::test]
async fn test_get_all_tasks() {
    let endpoint_url = format!("{}/task", *URL);
    let client = Client::new();
    let res = client
        .get(&endpoint_url)
        .send()
        .await
        .expect("Failed to get tasks");

    assert!(res.status().is_success());
}

#[tokio::test]
async fn test_delete_task() {
    let endpoint_url = format!("{}/task/1", *URL);
    let client = Client::new();
    let res = client
        .delete(&endpoint_url)
        .send()
        .await
        .expect("Failed to delete task");

    assert!(res.status().is_success());
}
