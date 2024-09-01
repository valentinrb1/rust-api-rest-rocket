use rocket::async_trait;
use crate::{business::dtos::mealplan_dto::{MealPlanDTO, RecipeMealsDTO}, data_access::{repository::{mealplan_repository::MealPlanRepository, recipe_repository::RecipeRepository, recipemeal_repository::MealRecipeRepository}, entities::{meal_plans::MealPlans, recipe_meals::RecipeMeals}}};
use validator::Validate;
use sqlx::{MySqlPool, Error};

#[derive(Debug)]
pub enum MealPlanError {
    ValidationError,
    DatabaseError(Error),
    DuplicateNameError,
    NotFound,
    NotImplemented
}

pub struct MealPlanService {
    pub mealplan_repository: MealPlanRepository,
    pub recipe_repository: RecipeRepository,
    pub recipemeal_repository: MealRecipeRepository
}

impl MealPlanService {
    pub fn new(db_pool: MySqlPool) -> Self {
        let mealplan_repository = MealPlanRepository::new(db_pool.clone());
        let recipe_repository = RecipeRepository::new(db_pool.clone());
        let recipemeal_repository = MealRecipeRepository::new(db_pool.clone());
        Self { mealplan_repository, recipe_repository, recipemeal_repository }
    }
}

#[async_trait]
pub trait MealPlanServiceTrait: Send + Sync {
    async fn add_mealplan(&self, meal_plan: MealPlanDTO) -> Result<(), MealPlanError>;
    async fn get_all_mealplan(&self) -> Result<Vec<MealPlanDTO>, MealPlanError>;
    async fn update_mealplan(&self, meal_plan: MealPlanDTO) -> Result<(), MealPlanError>;
    async fn delete_mealplan(&self, id: i64) -> Result<(), MealPlanError>;
    async fn check_recipe_existence(&self, recipes: &[RecipeMealsDTO]) -> Result<bool, MealPlanError>;
    fn map_mealplan(&self, mealplan_dto: &MealPlanDTO) -> MealPlans;
}

#[async_trait]
impl MealPlanServiceTrait for MealPlanService  {
    async fn add_mealplan(&self, meal_plan_dto: MealPlanDTO) -> Result<(), MealPlanError> {        
        if meal_plan_dto.validate().is_err() {
            return Err(MealPlanError::ValidationError);
        }
        
        if self.mealplan_repository.does_name_exist(&meal_plan_dto.name).await.map_err(MealPlanError::DatabaseError)? {
            return Err(MealPlanError::DuplicateNameError);
        }

        if !self.check_recipe_existence(&meal_plan_dto.recipes).await? {
            return Err(MealPlanError::NotFound);
        }

        let mealplan = self.map_mealplan(&meal_plan_dto);

        let result = self.mealplan_repository.add_mealplan(&mealplan).await;

        match result {
            Ok(_) => {
                let last_inserted_id: i64 = result.unwrap();
                
                let mut recipe_list = Vec::new();
    
                for recipe in &meal_plan_dto.recipes {
                    let new_recipe = RecipeMeals {
                        id_recipe_meal: 0,
                        day: recipe.day.clone(),
                        meal_type: recipe.meal_type.clone(),
                        recipes_id: recipe.id_recipe,
                        mealplan_id: last_inserted_id
                    };
                    
                    recipe_list.push(new_recipe);
                }
    
                match self.recipemeal_repository.add_recipe_meal(&recipe_list).await {
                    Ok(_) => Ok(()),
                    Err(err) => Err(MealPlanError::DatabaseError(err))
                }
            }
            Err(err) => {
                Err(MealPlanError::DatabaseError(err))
            }
        }
    }

    async fn get_all_mealplan(&self) -> Result<Vec<MealPlanDTO>, MealPlanError> {
        let meal_plans = self.mealplan_repository.get_all_mealplans().await.map_err(MealPlanError::DatabaseError)?;
        
        let mut meal_plan_dto: Vec<MealPlanDTO> = Vec::new();

        for meal_plan in meal_plans {
            let recipe_meals = self.recipemeal_repository.get_all_recipe_mealplan(meal_plan.id_mealplan).await.map_err(MealPlanError::DatabaseError)?;
    
            meal_plan_dto.push(MealPlanDTO {
                id_mealplan: meal_plan.id_mealplan,
                name: meal_plan.name,
                category: meal_plan.category,
                recipes: recipe_meals
                    .into_iter()
                    .map(|recipe_meals| RecipeMealsDTO {
                        id_recipe: recipe_meals.recipes_id,
                        day: recipe_meals.day,
                        meal_type: recipe_meals.meal_type
                    })
                    .collect(),
            });
        }
    
        Ok(meal_plan_dto)
    }

    async fn delete_mealplan(&self, id: i64) -> Result<(), MealPlanError> {
        self.recipemeal_repository.delete_recipemeal(id).await
            .map_err(|err| MealPlanError::DatabaseError(err))?;
    
        self.mealplan_repository.delete_mealplan(id).await
            .map_err(|err| MealPlanError::DatabaseError(err))?;
    
        Ok(())
    }

    async fn update_mealplan(&self, _meal_plan: MealPlanDTO) -> Result<(), MealPlanError> {
        Err(MealPlanError::NotImplemented)
    }

    async fn check_recipe_existence(&self, recipes: &[RecipeMealsDTO]) -> Result<bool, MealPlanError> {
        for recipe in recipes {
            if !self.recipe_repository.recipe_exist(recipe.id_recipe).await.map_err(MealPlanError::DatabaseError)? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn map_mealplan(&self, mealplan_dto: &MealPlanDTO) -> MealPlans {
        MealPlans {
            id_mealplan: 0,
            name: mealplan_dto.name.clone(),
            category: mealplan_dto.category.clone()
        }
    }
}