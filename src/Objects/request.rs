use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Calculation {
    Area,
    Perimeter,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "shape")]
pub enum Shape {
    Circle { radius: f64 },
    Rectangle { length: f64, width: f64 },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub calculation: Calculation,
    #[serde(flatten)]
    pub shape: Shape,
}
