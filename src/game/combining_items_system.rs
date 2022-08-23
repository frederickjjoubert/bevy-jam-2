use crate::Commands;
use crate::config::data_recipes::RecipesData;
use crate::game::items::{Item, ItemId};
use crate::game::recipes::Recipe;


/// === Helpers ===
// Sorry the parameter names aren't the greatest here, over_item is the item that the dragged_item is currently 'hovering' over.
pub fn try_get_recipe(data: &RecipesData, dragged_item_id: ItemId, over_item_id: ItemId) -> Option<&Recipe> {
    let mut recipe_has_dragged_item: bool = false;
    let mut recipe_has_over_item: bool = false;

    let mut recipe_clone: Option<&Recipe> = None;

    data.recipes.iter().for_each(|recipe| {
        recipe.ingredients.iter().for_each(|ingredient| {
            if ingredient.item_id == dragged_item_id {
                recipe_has_dragged_item = true;
            }
            if ingredient.item_id == over_item_id {
                recipe_has_over_item = true;
            }

            if recipe_has_dragged_item && recipe_has_over_item {
                recipe_clone = Some(recipe);
            }

            recipe_has_dragged_item = false;
            recipe_has_over_item = false;
        });
    });

    recipe_clone
}

/// === Systems ===
pub fn combine_items_system(mut commands: Commands) {}
