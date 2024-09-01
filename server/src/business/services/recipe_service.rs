use crate::{data_access::{repository::{recipe_repository::RecipeRepository, recipeingredients_repository::RecipeIngredientsRepository, ingredient_repository::IngredientRepository}, entities::{recipes::Recipe, recipe_ingredients::RecipeIngredients}}, business::dtos::recipe_dto::{RecipeDTO, IngredientAmount}};
use rocket::async_trait;
use sqlx::{Error, MySqlPool};
use validator::Validate;

#[derive(Debug)]
pub enum RecipeError {
    ValidationError,
    DatabaseError(Error),
    DuplicateNameError,
    NotFound,
    NotImplemented
}

pub struct RecipeService {
    pub ingredient_repository: IngredientRepository,
    pub recipe_repository: RecipeRepository,
    pub recipeingredients_repository: RecipeIngredientsRepository
}

impl RecipeService {
    pub fn new(db_pool: MySqlPool) -> Self {
        let ingredient_repository = IngredientRepository::new(db_pool.clone());
        let recipe_repository = RecipeRepository::new(db_pool.clone());
        let recipeingredients_repository = RecipeIngredientsRepository::new(db_pool.clone());
        Self {
            ingredient_repository,
            recipe_repository,
            recipeingredients_repository
        }
    }
}

#[async_trait]
pub trait RecipeServiceTrait: Send + Sync {
    async fn add_recipe(&self, recipe: RecipeDTO) -> Result<(), RecipeError>;
    async fn get_all_recipe(&self) -> Result<Vec<RecipeDTO>, RecipeError>;
    async fn update_recipe(&self, recipe: RecipeDTO) -> Result<(), RecipeError>;
    async fn delete_recipe(&self, id: i64) -> Result<(), RecipeError>;
    async fn check_ingredients_existence(&self, ingredients: &[IngredientAmount]) -> Result<bool, RecipeError>;
    fn map_recipe(&self, recipe_dto: &RecipeDTO) -> Recipe;
}

#[async_trait]
impl RecipeServiceTrait for RecipeService  {
    async fn add_recipe(&self, recipe_dto: RecipeDTO) -> Result<(), RecipeError> {
        if recipe_dto.validate().is_err() {
            return Err(RecipeError::ValidationError);
        }
        
        if self.recipe_repository.does_name_exist(&recipe_dto.name).await.map_err(RecipeError::DatabaseError)? {
            return Err(RecipeError::DuplicateNameError);
        }

        if !self.check_ingredients_existence(&recipe_dto.ingredients).await? {
            return Err(RecipeError::NotFound);
        }

        let recipe = self.map_recipe(&recipe_dto);
                
        let result = self.recipe_repository.add_recipe(&recipe).await;

        match result {
            Ok(_) => {
                let last_inserted_id: i64 = result.unwrap();
                
                let mut recipe_ingredients_list = Vec::new();
    
                for ingredient_amount in &recipe_dto.ingredients {
                    let new_recipe_ingredient = RecipeIngredients {
                        id_recipe_ingredients: 0,
                        amount: ingredient_amount.amount,
                        unit: ingredient_amount.unit.clone(),
                        ingredients_id_ingredient: ingredient_amount.id_ingredient,
                        recipes_id_recipe: last_inserted_id,
                    };
                    
                    recipe_ingredients_list.push(new_recipe_ingredient);
                }
    
                match self.recipeingredients_repository.add_ingredient_recipe(&recipe_ingredients_list).await {
                    Ok(_) => Ok(()),
                    Err(err) => Err(RecipeError::DatabaseError(err))
                }
            }
            Err(err) => {
                Err(RecipeError::DatabaseError(err))
            }
        }
    }

    async fn get_all_recipe(&self) -> Result<Vec<RecipeDTO>, RecipeError> {
        let recipes = self.recipe_repository.get_all_recipes().await.map_err(RecipeError::DatabaseError)?;
        
        let mut recipes_dto: Vec<RecipeDTO> = Vec::new();

        for recipe in recipes {
            let recipe_ingredients = self.recipeingredients_repository.get_all_recipe_ingredients(recipe.id_recipe).await.map_err(RecipeError::DatabaseError)?;
    
            recipes_dto.push(RecipeDTO {
                id_recipe: recipe.id_recipe,
                name: recipe.name,
                category: recipe.category,
                instructions: recipe.instructions,
                ingredients: recipe_ingredients
                    .into_iter()
                    .map(|recipe_ingredient| IngredientAmount {
                        id_ingredient: recipe_ingredient.ingredients_id_ingredient,
                        amount: recipe_ingredient.amount,
                        unit: recipe_ingredient.unit
                    })
                    .collect(),
            });
        }
    
        Ok(recipes_dto)
    }

    async fn update_recipe(&self, _recipe: RecipeDTO) -> Result<(), RecipeError> {
        Err(RecipeError::NotImplemented)
    }

    async fn delete_recipe(&self, id: i64) -> Result<(), RecipeError> {
        if self.recipe_repository.is_recipe_in_meal_plan(id).await.map_err(RecipeError::DatabaseError)? {
            return Err(RecipeError::DuplicateNameError);
        }

        match self.recipe_repository.delete_recipe(id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(RecipeError::DatabaseError(err))
        }
    }

    async fn check_ingredients_existence(&self, ingredients: &[IngredientAmount]) -> Result<bool, RecipeError> {
        for ingredient in ingredients {
            if !self.ingredient_repository.ingredient_exist(ingredient.id_ingredient).await.map_err(RecipeError::DatabaseError)? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn map_recipe(&self, recipe_dto: &RecipeDTO) -> Recipe {
        Recipe {
            id_recipe: 0,
            name: recipe_dto.name.clone(),
            category: recipe_dto.category.clone(),
            instructions: recipe_dto.instructions.clone()
        }
    }
}