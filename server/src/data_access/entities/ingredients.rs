use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Ingredient {
    #[serde(rename = "idIngredient")]
    pub id_ingredient: i64,
    pub name: String,
    pub proteins: f64,
    pub carbs: f64,
    pub fats: f64
}