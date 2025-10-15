pub struct Exercise {
    name: ExerciseName,
    sets: Vec<ExerciseSet>,
    muscle_: Vec<Muscle>,
}

pub struct ExerciseSet {
    reps: ExerciseReps,
    weight: ExerciseWeight,
}

impl ExerciseSet {
    pub fn new(reps: u16, weight: u16) -> Self {
        Self { reps: ExerciseReps(reps), weight: ExerciseWeight(weight) }
    }
}

pub struct ExerciseName(String);
impl ExerciseName {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
}

pub struct ExerciseReps(u16);
pub struct ExerciseWeight(u16);
impl ExerciseWeight {
    pub fn new(weight: u16) -> Self {
        Self(weight)
    }   
}

pub struct Muscle {
    group: MuscleGroup,
    activation: MuscleActivation
}

pub enum MuscleActivation {
    None,
    Low,
    Medium,
    High,
    VeryHigh
}

pub enum MuscleGroup {
    Back,
    Chest,
    Arms,
    Legs
}