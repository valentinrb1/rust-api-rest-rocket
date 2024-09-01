use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Recipe {
    #[serde(rename = "idRecipe")]
    pub id_recipe: i64,
    pub name: String,
    pub category: String,
    pub instructions: String
}