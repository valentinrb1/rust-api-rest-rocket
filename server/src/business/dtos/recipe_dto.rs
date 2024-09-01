use validator::Validate;
use serde::{Deserialize, Serialize};

use crate::data_access::entities::recipes::Recipe;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct RecipeDTO {
    pub id_recipe: i64,

    #[validate(length(min = 1, max = 45))]
    pub name: String,

    #[validate(length(min = 1, max = 45))]
    pub category: String,

    #[validate(length(min = 1, max = 1000))]
    pub instructions: String,

    pub ingredients: Vec<IngredientAmount>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IngredientAmount {
    pub id_ingredient: i64,
    pub amount: f64,
    pub unit: String
}

impl From<Recipe> for RecipeDTO {
    fn from(recipe: Recipe) -> Self {
        RecipeDTO {
            id_recipe: recipe.id_recipe,
            name: recipe.name,
            category: recipe.category,
            instructions: recipe.instructions,
            ingredients: vec![]
        }
    }
}