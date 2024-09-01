use sqlx::{MySql, MySqlPool, Error};

use crate::data_access::entities::meal_plans::MealPlans;
pub struct MealPlanRepository {
    pub db_pool: MySqlPool,
}

impl MealPlanRepository {
    pub fn new(db_pool: MySqlPool) -> Self {
        Self { db_pool }
    }

    pub async fn add_mealplan(&self, meal_plan: &MealPlans) -> Result<i64, Error> {
        let mut transaction = self.db_pool.begin().await?;
        
        let meal_plan_id: i64 = match sqlx::query(r#"
            INSERT INTO MealPlans (Name, Category)
            VALUES (?, ?)
        "#)
        .bind(&meal_plan.name)
        .bind(&meal_plan.category)
        .execute(&mut *transaction)
        .await
        {
            Ok(result) => result.last_insert_id() as i64,
            Err(err) => {
                transaction.rollback().await?;
                log::error!("Error adding meal plan to the database: {}", err);
                return Err(err);
            }
        };
    
        transaction.commit().await?;
        Ok(meal_plan_id)
    }

    pub async fn get_all_mealplans(&self) -> Result<Vec<MealPlans>, Error> {
        match sqlx::query_as::<MySql, MealPlans>("SELECT IdMealPlan as id_mealplan, name, category FROM mealplans")
            .fetch_all(&self.db_pool)
            .await
        {
            Ok(meal_plans) => Ok(meal_plans),
            Err(err) => {
                log::error!("Error retrieving meal plans from the database: {}", err);
                Err(err)
            }
        }
    }

    pub async fn delete_mealplan(&self, id: i64) -> Result<(), Error> {
        let mut transaction = self.db_pool.begin().await?;

        let query = "DELETE FROM MealPlans WHERE idMealPlan = ?";
    
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
                log::error!("Error deleting meal plan in the database: {}", err);
                Err(err)
            }
        }
    }

    pub async fn does_name_exist(&self, name: &str) -> Result<bool, Error> {
        let query = "SELECT COUNT(*) FROM mealplans WHERE name = ?";
        let count: i64 = sqlx::query_scalar(query)
            .bind(name)
            .fetch_one(&self.db_pool)
            .await?;

        Ok(count > 0)
    }

    pub async fn does_name_exist_and_id(&self, id_meal_plan: &i32, name: &str) -> Result<bool, Error> {
        let query = "SELECT COUNT(*) FROM MealPlans WHERE Name = ? AND idMealPlan != ?";
        let count: i64 = sqlx::query_scalar(query)
            .bind(name)
            .bind(id_meal_plan)
            .fetch_one(&self.db_pool)
            .await?;
    
        Ok(count > 0)
    }
}