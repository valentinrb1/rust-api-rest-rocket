use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct RecipeMeals {
    #[serde(rename = "idRecipeMeal")]
    pub id_recipe_meal: i64,
    pub day: String,
    pub meal_type: String,
    #[serde(rename = "Recipes_idRecipe")]
    pub recipes_id: i64,
    #[serde(rename = "MealPlans_idMealPlan")]
    pub mealplan_id: i64
}