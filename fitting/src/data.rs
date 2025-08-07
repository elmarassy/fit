use intermediate_representation::Float;

#[derive(Debug, Clone)]
pub enum Distribution {
    Uniform(Float, Float),
    Exponential(Float, Option<[Float; 2]>),
    Gaussian(Float, Float, Option<[Float; 2]>),
}

#[derive(Debug, Clone)]
pub struct Data {
    name: String,
    distribution: Distribution,
}

impl Data {
    pub fn new(name: &str, distribution: Distribution) -> Self {
        Self {
            name: name.to_string(),
            distribution,
        }
    }
}
