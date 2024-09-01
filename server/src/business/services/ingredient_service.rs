use rocket::async_trait;
use crate::business::dtos::ingredient_dto::IngredientDTO;
use crate::data_access::entities::ingredients::Ingredient;
use crate::data_access::repository::ingredient_repository::IngredientRepository;
use validator::Validate;
use sqlx::{MySqlPool, Error};

#[derive(Debug)]
pub enum IngredientError {
    ValidationError,
    DatabaseError(Error),
    DuplicateNameError,
    NotFound
}

pub struct IngredientService {
    pub ingredient_repository: IngredientRepository,
}

impl IngredientService {
    pub fn new(db_pool: MySqlPool) -> Self {
        let ingredient_repository = IngredientRepository::new(db_pool.clone());
        Self { ingredient_repository }
    }
}

#[async_trait]
pub trait IngredientServiceTrait: Send + Sync {
    async fn add_ingredient(&self, ingredient: IngredientDTO) -> Result<(), IngredientError>;
    async fn get_all_ingredients(&self) -> Result<Vec<IngredientDTO>, IngredientError>;
    async fn update_ingredient(&self, ingredient: IngredientDTO) -> Result<(), IngredientError>;
    async fn delete_ingredient(&self, id: i64) -> Result<(), IngredientError>;
}

#[async_trait]
impl IngredientServiceTrait for IngredientService  {
    async fn add_ingredient(&self, ingredient: IngredientDTO) -> Result<(), IngredientError> {
        if ingredient.validate().is_err() {
            return Err(IngredientError::ValidationError);
        }
        
        if self.ingredient_repository.does_name_exist(&ingredient.name).await.map_err(IngredientError::DatabaseError)? {
            return Err(IngredientError::DuplicateNameError);
        }

        let new_ingredient = Ingredient {
            id_ingredient: 0,
            name: ingredient.name.clone(),
            proteins: ingredient.proteins,
            carbs: ingredient.carbs,
            fats: ingredient.fats
        };

        match self.ingredient_repository.add_ingredient(&new_ingredient).await {
            Ok(_) => Ok(()),
            Err(err) => Err(IngredientError::DatabaseError(err))
        }
    }
     
    async fn get_all_ingredients(&self) -> Result<Vec<IngredientDTO>, IngredientError> {
        let ingredients = self.ingredient_repository.get_all_ingredients().await.map_err(IngredientError::DatabaseError)?;
    
        let ingredients_dto: Vec<IngredientDTO> = ingredients.into_iter().map(|ingredient| ingredient.into()).collect();
    
        Ok(ingredients_dto)
    }

     
    async fn update_ingredient(&self, ingredient_dto: IngredientDTO) -> Result<(), IngredientError> {
        if ingredient_dto.validate().is_err() {
            return Err(IngredientError::ValidationError);
        }
        
        if self.ingredient_repository.does_name_exist_and_id(&ingredient_dto.id_ingredient, &ingredient_dto.name).await.map_err(IngredientError::DatabaseError)? {
            return Err(IngredientError::DuplicateNameError);
        }
        
        match self.ingredient_repository.get_by_id(&ingredient_dto.id_ingredient).await {
            Ok(Some(mut ingredient)) => {
                ingredient.name = ingredient_dto.name;
                ingredient.carbs = ingredient_dto.carbs;
                ingredient.proteins = ingredient_dto.proteins;
                ingredient.fats = ingredient_dto.fats;

                match self.ingredient_repository.update_ingredient(&ingredient).await {
                    Ok(_) => Ok(()),
                    Err(err) => Err(IngredientError::DatabaseError(err)),
                }
            }
            Ok(None) => {
                return Err(IngredientError::NotFound);
            }
            Err(err) => {
                log::error!("Error: {}", err);
                return Err(IngredientError::DatabaseError(err));
            }
        }
    }

    async fn delete_ingredient(&self, id: i64) -> Result<(), IngredientError> {
        if self.ingredient_repository.is_ingredient_in_recipe_ingredient(id).await.map_err(IngredientError::DatabaseError)? {
            return Err(IngredientError::DuplicateNameError);
        }

        match self.ingredient_repository.delete_ingredient(id).await {
            Ok(_) => Ok(()),
            Err(err) => Err(IngredientError::DatabaseError(err))
        }
    }
}