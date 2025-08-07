#![allow(dead_code)]
use code_generation::define_model;
use fitting::data::Data;
use fitting::parameter::Parameter;

#[define_model]
mod gaussian {

    #[derive(Debug)]
    pub struct Gaussian {
        pub mu: Parameter,
        pub sigma: Parameter,
        pub x: Data,
    }

    pub fn distribution(mu: Float, sigma: Float, x: Float) -> Float {
        let norm = (2.0 * Float::PI).powf(-0.5) / sigma;
        norm * (-((x - mu) / sigma).powi(2) / 2.0).exp()
    }

    pub fn likelihood(mu: Float, sigma: Float, x: Float) -> Float {
        let norm = (2.0 * Float::PI).powf(-0.5) / sigma;
        norm.ln() + (-((x - mu) / sigma).powi(2) / 2.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    #[allow(unused_imports)]
    use rayon::prelude::*;
    use std::time;
    #[test]
    fn it_works() {
        // let b = Gaussian::_value_and_gradient([1.0f64, 1.2f64], [0.0f64]);
        // println!("{:?}", b);
        // println!("DONE");
        // assert_eq!(1.0, b.0);
        let n = 1000000;
        let mut rng = rand::rng();
        let b = time::Instant::now();
        let mut c = 0.0;
        let mut parameters = [1.0, 0.0];
        for _ in 0..1 {
            let vec: Vec<[Float; 1]> = (0..n).map(|_| rng.random()).collect();
            for i in 0..500 {
                let k: Float = vec
                    .par_iter()
                    .map(|f| gaussian::_likelihood(parameters, *f).0)
                    .sum();
                c += k;
                // c += k.sin();
                // parameters[0] += k.cos() / 10000.0;
            }
        }
        println!("Done: {} {:?}", c, b.elapsed());
        println!("Done: {}", c);
        println!("{:?}", gaussian::_likelihood([1.0, 0.0], [0.3]));
        assert_eq!(0.0, 1.0);
    }
}
