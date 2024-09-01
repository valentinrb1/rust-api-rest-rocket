use validator::Validate;
use serde::{Deserialize, Serialize};

use crate::data_access::entities::ingredients::Ingredient;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct IngredientDTO {
    pub id_ingredient: i64,

    #[validate(length(min = 1, max = 45))]
    pub name: String,

    #[validate(range(min = 0.0))]
    pub proteins: f64,

    #[validate(range(min = 0.0))]
    pub carbs: f64,

    #[validate(range(min = 0.0))]
    pub fats: f64
}

impl From<Ingredient> for IngredientDTO {
    fn from(ingredient: Ingredient) -> Self {
        IngredientDTO {
            id_ingredient: ingredient.id_ingredient,
            name: ingredient.name,
            proteins: ingredient.proteins,
            carbs: ingredient.carbs,
            fats: ingredient.fats,
        }
    }
}
