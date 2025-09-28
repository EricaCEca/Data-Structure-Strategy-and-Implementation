use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
pub id: u64,
pub title: String,
pub description: String,
pub category: String,
pub price: f64,
pub tags: Vec<String>,
}