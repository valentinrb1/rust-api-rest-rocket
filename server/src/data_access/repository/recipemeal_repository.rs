use sqlx::{MySql, MySqlPool, Error};

use crate::data_access::entities::recipe_meals::RecipeMeals;

pub struct MealRecipeRepository {
    pub db_pool: MySqlPool,
}

impl MealRecipeRepository {
    pub fn new(db_pool: MySqlPool) -> Self {
        Self { db_pool }
    }

    pub async fn add_recipe_meal(&self, recipe_meals: &[RecipeMeals]) -> Result<(), sqlx::Error> {
        let mut transaction = self.db_pool.begin().await?;
        
        for meal in recipe_meals {
            match sqlx::query(r#"
                INSERT INTO RecipeMeals (Day, MealType, Recipes_idRecipe, MealPlans_idMealPlan)
                VALUES (?, ?, ?, ?)
            "#)
            .bind(&meal.day)
            .bind(&meal.meal_type)
            .bind(&meal.recipes_id)
            .bind(&meal.mealplan_id)
            .execute(&mut *transaction)
            .await
            {
                Ok(_) => {}
                Err(err) => {
                    transaction.rollback().await?;
                    log::error!("Error adding recipe meal to the database: {}", err);
                    return Err(err);
                }
            }
        }
    
        transaction.commit().await?;
        Ok(())
    }

    pub async fn get_all_recipe_mealplan(&self, id_mealplan: i64) -> Result<Vec<RecipeMeals>, Error> {
        let query = "SELECT idRecipeMeals as id_recipe_meal, day, mealtype as meal_type, Recipes_idRecipe as recipes_id, MealPlans_idMealPlan as mealplan_id FROM recipemeals WHERE MealPlans_idMealPlan = ?";
        
        match sqlx::query_as::<MySql, RecipeMeals>(query)
            .bind(id_mealplan)
            .fetch_all(&self.db_pool)
            .await
        {
            Ok(recipe_ingredients) => Ok(recipe_ingredients),
            Err(err) => {
                log::error!("Error retrieving recipes in meal plans from the database: {}", err);
                Err(err)
            }
        }
    }

    pub async fn delete_recipemeal(&self, id: i64) -> Result<(), Error> {
        let mut transaction = self.db_pool.begin().await?;

        let query = "DELETE FROM RecipeMeals WHERE MealPlans_idMealPlan = ?";
    
        match sqlx::query(query)
            .bind(id)
            .execute(&mut *transaction)
            .await
        {
            Ok(_) => {
                transaction.commit().await?;
                Ok(())
            }
            Err(err) => {
                transaction.rollback().await?;
                log::error!("Error deleting recipe meal in the database: {}", err);
                Err(err)
            }
        }
    }
}