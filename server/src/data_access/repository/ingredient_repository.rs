use sqlx::{MySql, MySqlPool, Error};
use crate::data_access::entities::ingredients::Ingredient; 

pub struct IngredientRepository {
    pub db_pool: MySqlPool,
}

impl IngredientRepository {
    pub fn new(db_pool: MySqlPool) -> Self {
        Self { db_pool }
    }

    pub async fn add_ingredient(&self, ingredient: &Ingredient) -> Result<(), Error> {
        let query = r#"
            INSERT INTO ingredients (idIngredient, Name, Proteins, Carbs, Fats)
            VALUES (?, ?, ?, ?, ?)
        "#;
    
        match sqlx::query(query)
            .bind(ingredient.id_ingredient)
            .bind(ingredient.name.as_str())
            .bind(ingredient.proteins)
            .bind(ingredient.carbs)
            .bind(ingredient.fats)
            .execute(&self.db_pool)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => {
                log::error!("Error adding ingredient to the database: {}", err);
                Err(err)
            }
        }
    }

    pub async fn get_all_ingredients(&self) -> Result<Vec<Ingredient>, Error> {
        match sqlx::query_as::<MySql, Ingredient>("SELECT IdIngredient as id_ingredient, name, proteins, carbs, fats FROM ingredients")
            .fetch_all(&self.db_pool)
            .await
        {
            Ok(ingredients) => Ok(ingredients),
            Err(err) => {
                log::error!("Error retrieving ingredients from the database: {}", err);
                Err(err)
            }
        }
    }

    pub async fn get_by_id(&self, id: &i64) -> Result<Option<Ingredient>, Error> {
        let query = "SELECT IdIngredient as id_ingredient, name, proteins, carbs, fats FROM ingredients WHERE idIngredient = ?";
        
        match sqlx::query_as::<MySql, Ingredient>(query)
            .bind(id)
            .fetch_optional(&self.db_pool)
            .await
        {
            Ok(ingredient) => Ok(ingredient),
            
            Err(err) => {
                Err(err)
            }
        }
    }

    pub async fn update_ingredient(&self, ingredient: &Ingredient) -> Result<(), Error> {
        let query = r#"
            UPDATE ingredients
            SET Name = ?, Proteins = ?, Carbs = ?, Fats = ?
            WHERE idIngredient = ?
        "#;
    
        match sqlx::query(query)
            .bind(ingredient.name.as_str())
            .bind(ingredient.proteins)
            .bind(ingredient.carbs)
            .bind(ingredient.fats)
            .bind(ingredient.id_ingredient)
            .execute(&self.db_pool)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => {
                log::error!("Error updating ingredient in the database: {}", err);
                Err(err)
            }
        }
    }

    pub async fn delete_ingredient(&self, ingredient_id: i64) -> Result<(), Error> {
        let query = "DELETE FROM Ingredients WHERE idIngredient = ?";
    
        match sqlx::query(query)
            .bind(ingredient_id)
            .execute(&self.db_pool)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => {
                log::error!("Error deleting ingredient in the database: {}", err);
                Err(err)
            }
        }
    }

    pub async fn does_name_exist(&self, name: &str) -> Result<bool, Error> {
        let query = "SELECT COUNT(*) FROM ingredients WHERE Name = ?";
        let count: i64 = sqlx::query_scalar(query)
            .bind(name)
            .fetch_one(&self.db_pool)
            .await?;

        Ok(count > 0)
    }

    pub async fn does_name_exist_and_id(&self, id_ingredient: &i64, name: &str) -> Result<bool, Error> {
        let query = "SELECT COUNT(*) FROM ingredients WHERE Name = ? AND idIngredient != ?";
        let count: i64 = sqlx::query_scalar(query)
            .bind(name)
            .bind(id_ingredient)
            .fetch_one(&self.db_pool)
            .await?;
    
        Ok(count > 0)
    }

    pub async fn is_ingredient_in_recipe_ingredient(&self, ingredient_id: i64,) -> Result<bool, Error> {
        let query = "SELECT COUNT(*) FROM recipeIngredients WHERE Ingredients_idIngredient = ?";
    
        let count: i64 = sqlx::query_scalar(query)
            .bind(ingredient_id)
            .fetch_one(&self.db_pool)
            .await?;
    
        Ok(count > 0)
    }

    pub async fn ingredient_exist(&self, id: i64) -> Result<bool, Error> {
        let query = "SELECT COUNT(*) FROM ingredients WHERE idIngredient = ?";
        let count: i64 = sqlx::query_scalar(query)
            .bind(id)
            .fetch_one(&self.db_pool)
            .await?;
    
        Ok(count > 0)
    }
}