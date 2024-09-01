use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::{get, post, put, delete};
use rocket::State;
use validator::Validate;

use crate::business::dtos::recipe_dto::RecipeDTO;
use crate::business::services::recipe_service::{RecipeServiceTrait, RecipeError};

#[post("/add/recipe", data = "<recipe_data>")]
pub async fn add_recipe(
    recipe_data: Json<RecipeDTO>,
    recipe_service: &State<Box<dyn RecipeServiceTrait + Send + Sync>>,
) -> Result<String, Status> {
    let recipe = recipe_data.into_inner();

    if recipe.validate().is_err() {
        return Err(Status::UnprocessableEntity);
    }

    match recipe_service.add_recipe(recipe).await {
        Ok(()) => Ok("Recipe added successfully".to_string()),
        Err(RecipeError::ValidationError) => Err(Status::UnprocessableEntity),
        Err(RecipeError::DatabaseError(_)) => Err(Status::InternalServerError),
        Err(RecipeError::DuplicateNameError) => Err(Status::Conflict),
        Err(RecipeError::NotFound) => Err(Status::NotFound),
        Err(RecipeError::NotImplemented) => Err(Status::NotImplemented)
    }
}

#[get("/get/recipe")]
pub async fn get_all_recipes(
    recipe_service: &State<Box<dyn RecipeServiceTrait + Send + Sync>>,
) -> Result<Json<Vec<RecipeDTO>>, Status> {
    match recipe_service.get_all_recipe().await {
        Ok(recipes) => Ok(Json(recipes)),
        Err(RecipeError::DatabaseError(_)) => Err(Status::InternalServerError),
        _ => Err(Status::InternalServerError)
    }
}

#[delete("/delete/recipe/<id>")]
pub async fn delete_recipe(
    id: i64,
    recipe_service: &State<Box<dyn RecipeServiceTrait + Send + Sync>>,
) -> Result<String, Status> {
    match recipe_service.delete_recipe(id).await {
        Ok(()) => Ok("Recipe deleted successfully".to_string()),
        Err(RecipeError::DatabaseError(_)) => Err(Status::InternalServerError),
        Err(RecipeError::NotFound) => Err(Status::NotFound),
        _ => Err(Status::InternalServerError),
    }
}

#[put("/update/recipe", data = "<recipe_data>")]
pub async fn update_recipe(
    recipe_data: Json<RecipeDTO>,
    recipe_service: &State<Box<dyn RecipeServiceTrait + Send + Sync>>,
) -> Result<String, Status> {
    let recipe = recipe_data.into_inner();

    if recipe.validate().is_err() {
        return Err(Status::UnprocessableEntity);
    }

    match recipe_service.update_recipe(recipe).await {
        Ok(()) => Ok("Recipe updated successfully".to_string()),
        Err(RecipeError::ValidationError) => Err(Status::UnprocessableEntity),
        Err(RecipeError::DatabaseError(_)) => Err(Status::InternalServerError),
        Err(RecipeError::DuplicateNameError) => Err(Status::Conflict),
        Err(RecipeError::NotFound) => Err(Status::NotFound),
        Err(RecipeError::NotImplemented) => Err(Status::NotImplemented)
    }
}