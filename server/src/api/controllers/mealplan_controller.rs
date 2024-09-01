use rocket::serde::json::Json;
use rocket::http::Status;
use rocket::{get, post, put, delete};
use rocket::State;
use validator::Validate;

use crate::business::dtos::mealplan_dto::MealPlanDTO;
use crate::business::services::mealplan_service::{MealPlanServiceTrait, MealPlanError};

#[post("/add/mealplan", data = "<mealplan_data>")]
pub async fn add_mealplan(
    mealplan_data: Json<MealPlanDTO>,
    mealplan_service: &State<Box<dyn MealPlanServiceTrait + Send + Sync>>,
) -> Result<String, Status> {
    let mealplan = mealplan_data.into_inner();

    if mealplan.validate().is_err() {
        return Err(Status::UnprocessableEntity);
    }

    match mealplan_service.add_mealplan(mealplan).await {
        Ok(()) => Ok("mealplan added successfully".to_string()),
        Err(MealPlanError::ValidationError) => Err(Status::UnprocessableEntity),
        Err(MealPlanError::DatabaseError(_)) => Err(Status::InternalServerError),
        Err(MealPlanError::DuplicateNameError) => Err(Status::Conflict),
        Err(MealPlanError::NotFound) => Err(Status::NotFound),
        Err(MealPlanError::NotImplemented) => Err(Status::NotImplemented)
    }
}

#[get("/get/mealplan")]
pub async fn get_all_mealplans(
    mealplan_service: &State<Box<dyn MealPlanServiceTrait + Send + Sync>>,
) -> Result<Json<Vec<MealPlanDTO>>, Status> {
    match mealplan_service.get_all_mealplan().await {
        Ok(mealplans) => Ok(Json(mealplans)),
        Err(MealPlanError::DatabaseError(_)) => Err(Status::InternalServerError),
        _ => Err(Status::InternalServerError)
    }
}

#[delete("/delete/mealplan/<id>")]
pub async fn delete_mealplan(
    id: i64,
    mealplan_service: &State<Box<dyn MealPlanServiceTrait + Send + Sync>>,
) -> Result<String, Status> {
    match mealplan_service.delete_mealplan(id).await {
        Ok(()) => Ok("Meal plan deleted successfully".to_string()),
        Err(MealPlanError::DatabaseError(_)) => Err(Status::InternalServerError),
        Err(MealPlanError::NotFound) => Err(Status::NotFound),
        _ => Err(Status::InternalServerError),
    }
}

#[put("/update/mealplan", data = "<mealplan_data>")]
pub async fn update_mealplan(
    mealplan_data: Json<MealPlanDTO>,
    mealplan_service: &State<Box<dyn MealPlanServiceTrait + Send + Sync>>,
) -> Result<String, Status> {
    let mealplan = mealplan_data.into_inner();

    if mealplan.validate().is_err() {
        return Err(Status::UnprocessableEntity);
    }

    match mealplan_service.update_mealplan(mealplan).await {
        Ok(()) => Ok("Meal plan updated successfully".to_string()),
        Err(MealPlanError::ValidationError) => Err(Status::UnprocessableEntity),
        Err(MealPlanError::DatabaseError(_)) => Err(Status::InternalServerError),
        Err(MealPlanError::DuplicateNameError) => Err(Status::Conflict),
        Err(MealPlanError::NotFound) => Err(Status::NotFound),
        Err(MealPlanError::NotImplemented) => Err(Status::NotImplemented)
    }
}