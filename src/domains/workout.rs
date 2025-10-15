use std::clone;

use crate::domains::exercise::{self, Exercise, ExerciseId};

use ::uuid::Uuid;
use ::chrono::{DateTime, Utc};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Workout {
    pub name: WorkoutName,
    pub id: WorkoutId,
    pub date: DateTime<Utc>,
    pub exercises: Vec<Exercise>
}

impl Workout {
    pub fn new(name: &str) -> Self {
        Self { name: WorkoutName::new(name), id: WorkoutId::new(), date: Utc::now(), exercises: Vec::new() }
    }
    
    pub fn add_exercise(&mut self, exercise: Exercise) {
        self.exercises.push(exercise);
    }

    pub fn remove_exercise(&mut self, id: ExerciseId) -> anyhow::Result<(), RemoveExerciseError> {
        Ok(())
    }
}

#[derive(thiserror::Error, Debug, Clone)]
#[error("Could not remove the new exercise: {0}")]
pub struct RemoveExerciseError(String);

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct WorkoutName(String);
impl WorkoutName {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
    pub fn get(&self) -> String {
        self.0.clone().to_string()
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct WorkoutId(Uuid);
impl WorkoutId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    pub fn get(&self) -> String {
        self.0.to_string().clone()
    }
}