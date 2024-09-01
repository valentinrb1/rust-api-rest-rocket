use sqlx::{MySqlPool, MySql, Error};

use crate::data_access::entities::recipe_ingredients::RecipeIngredients;

pub struct RecipeIngredientsRepository {
    pub db_pool: MySqlPool,
}

impl RecipeIngredientsRepository {
    pub fn new(db_pool: MySqlPool) -> Self {
        Self { db_pool }
    }

    pub async fn add_ingredient_recipe(&self, recipe_ingredients: &[RecipeIngredients]) -> Result<(), sqlx::Error> {
        let mut transaction = self.db_pool.begin().await?;

        let query_ingredients = r#"
            INSERT INTO recipeIngredients (idRecipeIngredient, Amount, Unit, Ingredients_idIngredient, Recipes_idRecipe)
            VALUES (?, ?, ?, ?, ?)
        "#;

        for recipe_ingredient in recipe_ingredients {
            match sqlx::query(query_ingredients)
                .bind(recipe_ingredient.id_recipe_ingredients)
                .bind(recipe_ingredient.amount)
                .bind(recipe_ingredient.unit.clone())
                .bind(recipe_ingredient.ingredients_id_ingredient)
                .bind(recipe_ingredient.recipes_id_recipe)
                .execute(&mut *transaction)
                .await
            {
                Ok(_) => {}
                Err(err) => {
                    transaction.rollback().await?;
                    log::error!("Error adding recipe ingredient to the database: {}", err);
                    return Err(err);
                }
            }
        }
        
        transaction.commit().await?;
        Ok(())
    }

    pub async fn get_all_recipe_ingredients(&self, id_recipe: i64) -> Result<Vec<RecipeIngredients>, Error> {
        let query = "SELECT idRecipeIngredient as id_recipe_ingredients, amount, unit, Ingredients_idIngredient as ingredients_id_ingredient, Recipes_idRecipe as recipes_id_recipe FROM recipeingredients WHERE Recipes_idRecipe = ?";
        
        match sqlx::query_as::<MySql, RecipeIngredients>(query)
            .bind(id_recipe)
            .fetch_all(&self.db_pool)
            .await
        {
            Ok(recipe_ingredients) => Ok(recipe_ingredients),
            Err(err) => {
                log::error!("Error retrieving recipe ingredients from the database: {}", err);
                Err(err)
            }
        }
    }
}