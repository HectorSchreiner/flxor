use dioxus::{html::{h1, input}, prelude::*};

use crate::domains::workout::*;

#[component]
pub fn Main() -> Element {    
    rsx! {
        div {
            id: "title",
            h1 { "FLXOR" }
        },
        div { 
            
         }
    }
}

#[component]
pub fn WorkoutDisplay(workout: Workout) -> Element {
    // Note: To use the data inside the rsx! macro, you need to access the inner String
    // for structs like WorkoutName.
    rsx! {
        div { 
            id: "workout-card",
            h2 { "{workout.name.as_string()}" }
        },
        h3 { "Exercises:" },
        ul { 
            id: "exercise-list",
            for exercise in workout.exercises.iter() {
                for group in exercise.muscle_groups.iter() {
                    p { id: "muscle-group", "Muscle group: {group.group()}" }
                    p { id: "muscle-tag", "Activation Level: {group.activation()} " }
                }
                li {
                    span { id: "exercise-item", "{exercise.name.as_string()}" }
                    for set in exercise.sets.iter() {
                        div { 
                            span { id:"reps", "Reps: {set.reps.as_u16()} " }
                            span { id:"weight", "Weight: {set.weight.as_u16()} kg " }
                        }
                    }
                }
            }
        }
    }
}


