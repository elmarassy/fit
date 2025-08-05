#[allow(warnings)]
use code_generation::define_model;
use fitting::data::Data;
use fitting::parameter::Parameter;
use intermediate_representation::expression::Number;
use rand::Rng;
use std::f64;

#[define_model]
mod gaussian {
    #[derive(Debug)]
    pub struct Gaussian {
        pub mu: Parameter,
        pub sigma: Parameter,
        pub x: Data,
    }

    pub fn distribution(mu: Number, sigma: Number, x: Number) -> Number {
        let norm = (2.0 * 3.1415926535f64).powf(0.0 - 0.5) / sigma;
        norm.ln() + (0.0 - ((x - mu) / sigma).powi(2) / 2.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let b = Gaussian::_value_and_gradient([1.0f64, 1.2f64], [0.0f64]);
        // println!("{:?}", b);
        // println!("DONE");
        // assert_eq!(1.0, b.0);
        let n = 1000000;
        let mut rng = rand::rng();
        let vec: Vec<[f64; 1]> = (0..n).map(|_| rng.random()).collect();

        let mut b = 0.0;

        for i in 0..1000 {
            let k: f64 = vec
                .iter()
                .map(|f| gaussian::_value_and_gradient([0.0f64 + i as f64, 1.0f64], *f).0)
                .sum();
            b += k;
        }
        println!("Done: {}", b);
        println!(
            "{:?}",
            gaussian::_value_and_gradient([1.0f64, 0.0f64], [0.3])
        );
        assert_eq!(0.0, 1.0);
    }
}
