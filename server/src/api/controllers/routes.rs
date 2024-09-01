use rocket::routes;
use rocket::Route;

use crate::api::controllers::mealplan_controller::delete_mealplan;
use crate::api::controllers::mealplan_controller::update_mealplan;
use crate::api::controllers::mealplan_controller::{ add_mealplan, get_all_mealplans };
use crate::api::controllers::recipe_controller::{ get_all_recipes, add_recipe, update_recipe, delete_recipe };
use crate::api::controllers::ingredient_controller::{add_ingredient, get_all_ingredients, update_ingredient, delete_ingredient};

pub fn routes() -> Vec<Route> {
    routes![add_ingredient, get_all_ingredients, update_ingredient, delete_ingredient, 
            add_recipe, get_all_recipes, update_recipe, delete_recipe,
            add_mealplan, get_all_mealplans, update_mealplan, delete_mealplan]
}
