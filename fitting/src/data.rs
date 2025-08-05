use intermediate_representation::expression::Number;

#[derive(Debug, Clone)]
pub enum Distribution {
    Uniform(Number, Number),
    Exponential(Number, Option<[Number; 2]>),
    Gaussian(Number, Number, Option<[Number; 2]>),
}

#[derive(Debug, Clone)]
pub struct Data {
    name: String,
    distribution: Distribution,
}
