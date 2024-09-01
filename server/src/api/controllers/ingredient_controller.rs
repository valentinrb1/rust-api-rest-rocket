use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::{get, post, put, delete};
use rocket::State;
use validator::Validate;

use crate::business::dtos::ingredient_dto::IngredientDTO;
use crate::business::services::ingredient_service::{IngredientServiceTrait, IngredientError};

#[post("/add/ingredient", data = "<ingredient_data>")]
pub async fn add_ingredient(
    ingredient_data: Json<IngredientDTO>,
    ingredient_service: &State<Box<dyn IngredientServiceTrait + Send + Sync>>,
) -> Result<String, Status> {
    let ingredient = ingredient_data.into_inner();

    if ingredient.validate().is_err() {
        return Err(Status::UnprocessableEntity);
    }

    match ingredient_service.add_ingredient(ingredient).await {
        Ok(()) => Ok("Ingredient added successfully".to_string()),
        Err(IngredientError::ValidationError) => Err(Status::UnprocessableEntity),
        Err(IngredientError::DatabaseError(_)) => Err(Status::InternalServerError),
        Err(IngredientError::DuplicateNameError) => Err(Status::Conflict),
        Err(IngredientError::NotFound) => Err(Status::NotFound)
    }
}

#[get("/get/ingredient")]
pub async fn get_all_ingredients(
    ingredient_service: &State<Box<dyn IngredientServiceTrait + Send + Sync>>,
) -> Result<Json<Vec<IngredientDTO>>, Status> {
    match ingredient_service.get_all_ingredients().await {
        Ok(ingredients) => Ok(Json(ingredients)),
        Err(IngredientError::DatabaseError(_)) => Err(Status::InternalServerError),
        _ => Err(Status::InternalServerError)
    }
}

#[put("/update/ingredient", data = "<ingredient_data>")]
pub async fn update_ingredient(
    ingredient_data: Json<IngredientDTO>,
    ingredient_service: &State<Box<dyn IngredientServiceTrait + Send + Sync>>,
) -> Result<String, Status> {
    let ingredient = ingredient_data.into_inner();

    if ingredient.validate().is_err() {
        return Err(Status::UnprocessableEntity);
    }

    match ingredient_service.update_ingredient(ingredient).await {
        Ok(()) => Ok("Ingredient updated successfully".to_string()),
        Err(IngredientError::ValidationError) => Err(Status::UnprocessableEntity),
        Err(IngredientError::DatabaseError(_)) => Err(Status::InternalServerError),
        Err(IngredientError::DuplicateNameError) => Err(Status::Conflict),
        Err(IngredientError::NotFound) => Err(Status::NotFound)
    }
}

#[delete("/delete/ingredient/<id>")]
pub async fn delete_ingredient(
    id: i64,
    ingredient_service: &State<Box<dyn IngredientServiceTrait + Send + Sync>>,
) -> Result<String, Status> {
    match ingredient_service.delete_ingredient(id).await {
        Ok(()) => Ok("Ingredient deleted successfully".to_string()),
        Err(IngredientError::DatabaseError(_)) => Err(Status::InternalServerError),
        Err(IngredientError::NotFound) => Err(Status::NotFound),
        _ => Err(Status::InternalServerError),
    }
}