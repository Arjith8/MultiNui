use uuid::Uuid;

#[derive(Debug)]
pub struct Goal {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

#[derive(Debug)]
pub struct GoalSheet {
    pub id: Uuid,
    pub name: String,
    pub goals: Vec<Goal>
}
