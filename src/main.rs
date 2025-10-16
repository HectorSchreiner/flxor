use dioxus::prelude::*;

pub mod domains;
pub mod repos;
pub mod frontend;

use dioxus_logger::tracing::Level;
use frontend::app::*;
use repos::sqlite::*;

use crate::domains::{exercise::{Exercise, ExerciseSet, Muscle, MuscleActivationLevel, MuscleGroup}, workout::Workout};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus_logger::init(Level::INFO).expect("logger failed to init");
    let db = SqliteDatabase::new("workout.db").expect("Database failed to init");

    dioxus::launch(App);
}

#[component]
fn App() -> Element {

    let mut workout = Workout::new("Upper Body Workout");
    
    let mut bench_press = Exercise::new("Bench Press").unwrap();
    bench_press.add_muscle_group(Muscle::new(MuscleGroup::Chest, MuscleActivationLevel::VeryHigh)).unwrap();
    bench_press.push_set(ExerciseSet::new(8, 100).unwrap()).unwrap();
    bench_press.push_set(ExerciseSet::new(7, 95).unwrap()).unwrap();

    let mut leg_press = Exercise::new("Leg Press").unwrap();
    leg_press.add_muscle_group(Muscle::new(MuscleGroup::Legs, MuscleActivationLevel::VeryHigh)).unwrap();
    leg_press.push_set(ExerciseSet::new(8, 120).unwrap()).unwrap();
    leg_press.push_set(ExerciseSet::new(7, 120).unwrap()).unwrap();


    workout.add_exercise(bench_press);
    workout.add_exercise(leg_press);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Main {  }
        WorkoutDisplay { workout }
    }
}


