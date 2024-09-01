use std::env;
use env_logger::Env;
use sqlx::mysql::MySqlPoolOptions;

use server::api;
use server::business::services::ingredient_service::{ IngredientServiceTrait, IngredientService};
use server::business::services::mealplan_service::{ MealPlanService, MealPlanServiceTrait};
use server::business::services::recipe_service::{ RecipeService, RecipeServiceTrait};

#[tokio::main]
async fn main() {    
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init(); 
    
    dotenv::from_filename("database.env").ok();
    let db_url: String = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            eprintln!("Error: DATABASE_URL environment variable not set");
            std::process::exit(1);
        }
    };

    if let Err(err) = run(db_url.to_string()).await {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

async fn run(db_url: String) -> Result<(), sqlx::Error> {
    let pool: sqlx::Pool<sqlx::MySql> = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    let ingredient_service: Box<dyn IngredientServiceTrait + Send + Sync> =
        Box::new(IngredientService::new(pool.clone()));

    let recipe_service: Box<dyn RecipeServiceTrait + Send + Sync> =
        Box::new(RecipeService::new(pool.clone()));

    let mealplan_service: Box<dyn MealPlanServiceTrait + Send + Sync> =
        Box::new(MealPlanService::new(pool.clone()));

    let rocket = rocket::build()
        .mount("/api", api::controllers::routes::routes())
        .manage(ingredient_service)
        .manage(recipe_service)
        .manage(mealplan_service);

    rocket.launch().await.unwrap();
    
    Ok(())
}