use thiserror::Error;

use crate::domains::workout::Workout;

pub trait WorkoutRepo {
    fn add_workout() -> anyhow::Result<(), WorkoutDatabaseError>;
    fn remove_workout() -> anyhow::Result<(), WorkoutDatabaseError>;
    fn list_workouts() -> anyhow::Result<Vec<Workout>, WorkoutDatabaseError>;
}

#[derive(Error, Debug)]
pub enum WorkoutDatabaseError {
    #[error("Failed to list workouts: {0}")]
    List(String), 

    #[error("Failed to remove workout: {0}")]
    Remove(String),

    #[error("Failed to add workout: {0}")]
    Add(String),
}
