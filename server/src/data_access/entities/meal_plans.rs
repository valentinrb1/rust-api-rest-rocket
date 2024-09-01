use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct MealPlans {
    #[serde(rename = "idMealPlan")]
    pub id_mealplan: i64,
    pub name: String,
    pub category: String
}