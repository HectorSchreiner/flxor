use crate::domains::exercise::Exercise;

use ::uuid::Uuid;
use ::chrono::{DateTime, Utc};

pub struct Workout {
    name: WorkoutName,
    id: WorkoutId,
    date: DateTime<Utc>,
    exercises: Vec<Exercise>
}

impl Workout {
    pub fn new(name: &str) -> Self {
        Self { name: WorkoutName::new(name), id: WorkoutId::new(), date: Utc::now(), exercises: Vec::new() }
    }
}

pub struct WorkoutName(String);
impl WorkoutName {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
}

pub struct WorkoutId(Uuid);
impl WorkoutId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}