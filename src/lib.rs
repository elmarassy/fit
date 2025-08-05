#![allow(dead_code)]
use code_generation::define_model;
use fitting::data::Data;
use fitting::parameter::Parameter;
use intermediate_representation::expression::Number;

#[define_model]
mod gaussian {

    #[derive(Debug)]
    pub struct Gaussian {
        pub mu: Parameter,
        pub sigma: Parameter,
        pub x: Data,
    }

    pub fn distribution(mu: Number, sigma: Number, x: Number) -> Number {
        let norm = (2.0 * 3.1415926535 as Number).powf(-0.5) / sigma;
        norm * (-((x - mu) / sigma).powi(2) / 2.0).exp()
    }

    pub fn likelihood(mu: Number, sigma: Number, x: Number) -> Number {
        let norm = (2.0 * 3.1415926535 as Number).powf(-0.5) / sigma;
        norm.ln() + (-((x - mu) / sigma).powi(2) / 2.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    #[allow(unused_imports)]
    use rayon::prelude;

    #[test]
    fn it_works() {
        // let b = Gaussian::_value_and_gradient([1.0f64, 1.2f64], [0.0f64]);
        // println!("{:?}", b);
        // println!("DONE");
        // assert_eq!(1.0, b.0);
        let n = 4000;
        let mut rng = rand::rng();
        let vec: Vec<[Number; 1]> = (0..n).map(|_| rng.random()).collect();
        // let b = time::Instant::now();
        let mut c = 0.0;
        for i in 0..1000 {
            let k: Number = vec
                .iter()
                .map(|f| gaussian::_likelihood([1.0 as Number, 0.0 + i as Number], *f).0)
                .sum();
            c += k;
        }
        // println!("Done: {} {:?}", c, b.elapsed());
        println!("Done: {}", c);
        println!("{:?}", gaussian::_likelihood([1.0, 0.0], [0.3]));
        assert_eq!(0.0, 1.0);
    }
}
