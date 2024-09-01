use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct MealPlanDTO {
    pub id_mealplan: i64,

    #[validate(length(min = 1, max = 45))]
    pub name: String,

    #[validate(length(min = 1, max = 45))]
    pub category: String,

    pub recipes: Vec<RecipeMealsDTO>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RecipeMealsDTO {
    pub id_recipe: i64,
    pub day: String,
    pub meal_type: String
}