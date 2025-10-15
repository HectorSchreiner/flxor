#[derive(Debug)]
pub struct Exercise {
    name: ExerciseName,
    sets: Vec<ExerciseSet>,
    muscle_groups: Vec<Muscle>,
}

impl Exercise {
    pub fn new(name: &str) -> anyhow::Result<Self, NewExerciseError> {
        Ok(Self {
            name: ExerciseName::new(name),
            sets: Vec::new(),
            muscle_groups: Vec::new()
        })
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Could not create the new exercise: {0}")]
pub struct NewExerciseError(String);

#[derive(Debug)]
pub struct ExerciseSet {
    reps: ExerciseReps,
    weight: ExerciseWeight,
}

impl ExerciseSet {
    pub fn new(reps: u16, weight: u16) -> Self {
        Self { reps: ExerciseReps(reps), weight: ExerciseWeight(weight) }
    }
}

#[derive(Debug)]
pub struct ExerciseName(String);
impl ExerciseName {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
}

#[derive(Debug)]
pub struct ExerciseReps(u16);
impl ExerciseReps {
    pub fn new(reps: u16) -> Self {
        Self(reps)
    }   
}

#[derive(Debug)]
pub struct ExerciseWeight(u16);
impl ExerciseWeight {
    pub fn new(weight: u16) -> Self {
        Self(weight)
    }   
}

#[derive(Debug)]
pub struct Muscle {
    group: MuscleGroup,
    activation: MuscleActivation
}

#[derive(Debug)]
pub enum MuscleActivation {
    None,
    Low,
    Medium,
    High,
    VeryHigh
}

#[derive(Debug)]
pub enum MuscleGroup {
    Back,
    Chest,
    Arms,
    Legs
}