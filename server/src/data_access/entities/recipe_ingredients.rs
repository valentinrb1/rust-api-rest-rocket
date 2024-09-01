use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct RecipeIngredients {
    #[serde(rename = "idRecipeIngredient")]
    pub id_recipe_ingredients: i64,
    pub amount: f64,
    pub unit: String,
    #[serde(rename = "Ingredients_idIngredient")]
    pub ingredients_id_ingredient: i64,
    #[serde(rename = "Recipes_idRecipe")]
    pub recipes_id_recipe: i64
}