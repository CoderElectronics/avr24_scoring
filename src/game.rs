use derivative::Derivative;
use uuid::Uuid;

#[derive(Clone, Derivative, serde::Deserialize, serde::Serialize)]
#[derivative(Debug, Default)]
pub struct ScoringAction {
    #[derivative(Default(value = "Uuid::new_v4().to_string()"))]
    pub id: String,

    #[derivative(Default(value = "String::from(\"N/A\")"))]
    pub description: String,
    pub name: String,
    pub phase: i32,

    pub pointvalue: i32, // set to -1 to use point stages instead

    #[derivative(Default(value = "10"))]
    pub max_count: i32, // if using point stages this is useless

    #[derivative(Default(value = "0"))]
    pub count: i32, // if using point stages this is the index of the pointstages vector

    #[derivative(Default(value = "Vec::new()"))]
    pub pointstages: Vec<i32>,
}
