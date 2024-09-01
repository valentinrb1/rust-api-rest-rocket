use sqlx::{MySql, MySqlPool, Error};

use crate::data_access::entities::recipes::Recipe;

pub struct RecipeRepository {
    pub db_pool: MySqlPool,
}

impl RecipeRepository {
    pub fn new(db_pool: MySqlPool) -> Self {
        Self { db_pool }
    }

    pub async fn add_recipe(&self, recipe: &Recipe) -> Result<i64, Error> {
        let mut transaction = self.db_pool.begin().await?;
        let query = r#"
            INSERT INTO Recipes (idRecipe, Name, Category, Instructions)
            VALUES (?, ?, ?, ?)
        "#;
    
        match sqlx::query(query)
            .bind(recipe.id_recipe)
            .bind(&recipe.name)
            .bind(&recipe.category)
            .bind(&recipe.instructions)
            .execute(&mut *transaction)
            .await
        {
            Ok(result) => {
                let id: i64 = result.last_insert_id() as i64;
                transaction.commit().await?;
                Ok(id)
            }
            Err(err) => {
                transaction.rollback().await?;
                log::error!("Error adding ingredient to the database: {}", err);
                Err(err)
            }
        }
    }

    pub async fn get_all_recipes(&self) -> Result<Vec<Recipe>, Error> {
        match sqlx::query_as::<MySql, Recipe>("SELECT IdRecipe as id_recipe, name, category, instructions FROM recipes")
            .fetch_all(&self.db_pool)
            .await
        {
            Ok(recipes) => Ok(recipes),
            Err(err) => {
                log::error!("Error retrieving recipes from the database: {}", err);
                Err(err)
            }
        }
    }

    pub async fn is_recipe_in_meal_plan(&self, recipe_id: i64,) -> Result<bool, Error> {
        let query = "SELECT COUNT(*) FROM recipeMeals WHERE Recipes_idRecipe = ?";
    
        let count: i64 = sqlx::query_scalar(query)
            .bind(recipe_id)
            .fetch_one(&self.db_pool)
            .await?;
    
        Ok(count > 0)
    }

    pub async fn delete_recipe(&self, recipe_id: i64) -> Result<(), Error> {
        let query = "DELETE FROM Recipes WHERE idRecipe = ?";
    
        match sqlx::query(query)
            .bind(recipe_id)
            .execute(&self.db_pool)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => {
                log::error!("Error deleting recipe in the database: {}", err);
                Err(err)
            }
        }
    }

    pub async fn does_name_exist(&self, name: &str) -> Result<bool, Error> {
        let query = "SELECT COUNT(*) FROM Recipes WHERE Name = ?";
        let count: i64 = sqlx::query_scalar(query)
            .bind(name)
            .fetch_one(&self.db_pool)
            .await?;

        Ok(count > 0)
    }

    pub async fn does_name_exist_and_id(&self, id_recipe: &i32, name: &str) -> Result<bool, Error> {
        let query = "SELECT COUNT(*) FROM Recipes WHERE Name = ? AND idRecipe != ?";
        let count: i64 = sqlx::query_scalar(query)
            .bind(name)
            .bind(id_recipe)
            .fetch_one(&self.db_pool)
            .await?;
    
        Ok(count > 0)
    }

    pub async fn recipe_exist(&self, id: i64) -> Result<bool, Error> {
        let query = "SELECT COUNT(*) FROM recipes WHERE idRecipe = ?";
        let count: i64 = sqlx::query_scalar(query)
            .bind(id)
            .fetch_one(&self.db_pool)
            .await?;
    
        Ok(count > 0)
    }
}