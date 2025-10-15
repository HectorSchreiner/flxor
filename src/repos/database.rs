use std::ops::Add;

use anyhow::{Error, Result};
use thiserror::Error;

pub trait WorkoutRepo {
    fn add_exercise() -> anyhow::Result<(), AddWorkoutError>;
    fn remove_exercise() -> anyhow::Result<(), RemoveWorkoutError>;
}

#[derive(Error, Debug)]
#[error["{0}, could not be added to the database"]]
pub struct AddWorkoutError(String);

#[derive(Error, Debug)]
#[error["{0}, could not be removed from the database"]]
pub struct RemoveWorkoutError(String);
