use std::fmt::Display;

use uuid::Uuid;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Exercise {
    pub id: ExerciseId,
    pub name: ExerciseName,
    pub sets: Vec<ExerciseSet>,
    pub muscle_groups: Vec<Muscle>,
}

impl Exercise {
    pub fn new(name: &str) -> anyhow::Result<Self, NewExerciseError> {
        Ok(Self {
            id: ExerciseId::new(),
            name: ExerciseName::new(name),
            sets: Vec::new(),
            muscle_groups: Vec::new()
        })
    }

    pub fn push_set(&mut self, set: ExerciseSet) -> anyhow::Result<(), NewSetError> {
        Ok(self.sets.push(set))
    }

    pub fn set_name(&mut self, new_name: ExerciseName) -> anyhow::Result<(), SetExerciseNameError> {
        Ok(self.name = new_name)
    }

    pub fn add_muscle_group(&mut self, muscle: Muscle) -> anyhow::Result<(), AddMuscleGroupError> {
        Ok(self.muscle_groups.push(muscle))
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Could not add the muscle: {0}")]
pub struct AddMuscleGroupError(String);

#[derive(thiserror::Error, Debug)]
#[error("Could not set the exercisename to: {0}")]
pub struct SetExerciseNameError(String);


#[derive(thiserror::Error, Debug)]
#[error("Could not create the new exercise: {0}")]
pub struct NewExerciseError(String);


#[derive(thiserror::Error, Debug)]
#[error("Could not add the new set: {0}")]
pub struct NewSetError(String);

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct ExerciseSet {
    pub reps: ExerciseReps,
    pub weight: ExerciseWeight,
}

impl ExerciseSet {
    pub fn new(reps: u16, weight: u16) -> anyhow::Result<Self, NewSetError> {
        Ok(Self { reps: ExerciseReps(reps), weight: ExerciseWeight(weight) })
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct ExerciseName(String);
impl ExerciseName {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }

    pub fn as_string(&self) -> String {
        self.0.clone().to_string()
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct ExerciseReps(u16);
impl ExerciseReps {
    pub fn new(reps: u16) -> Self {
        Self(reps)
    }
    
    pub fn as_u16(&self) -> u16 {
        self.0.clone()
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct ExerciseWeight(u16);
impl ExerciseWeight {
    pub fn new(weight: u16) -> Self {
        Self(weight)
    }   

    pub fn as_u16(&self) -> u16 {
        self.0.clone()
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct ExerciseId(Uuid);
impl ExerciseId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn as_string(&self) -> String {
        self.0.to_string().clone()
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Muscle {
    group: MuscleGroup,
    activation: MuscleActivationLevel
}
impl Muscle {
    pub fn new(group: MuscleGroup, activation: MuscleActivationLevel) -> Self {
        Self { group, activation }
    }

    pub fn group(&self) -> String {
        match self.group {
            MuscleGroup::Back => String::from("Back"),
            MuscleGroup::Chest => String::from("Chest"),
            MuscleGroup::Arms =>  String::from("Arms"),
            MuscleGroup::Legs =>  String::from("Legs")     
        }
    }

    pub fn activation(&self) -> String {
        match self.activation {
            MuscleActivationLevel::None => String::from("None"),
            MuscleActivationLevel::Low => String::from("Low"),
            MuscleActivationLevel::Medium => String::from("Medium"),
            MuscleActivationLevel::High => String::from("High"),
            MuscleActivationLevel::VeryHigh => String::from("Very High"),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum MuscleActivationLevel {
    None,
    Low,
    Medium,
    High,
    VeryHigh
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum MuscleGroup {
    Back,
    Chest,
    Arms,
    Legs
}