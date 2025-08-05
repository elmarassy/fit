#[allow(warnings)]
use code_generation::define_pdf;
use fitting::data::Data;
use fitting::parameter::Parameter;
use intermediate_representation::expression::Number;
use rand::Rng;
use std::f64;

define_pdf!(
    #[derive(Debug)]
    struct Gaussian {
        pub mu: Parameter,
        pub sigma: Parameter,
        pub x: Data,
    }

    fn distribution(mu: f64, sigma: f64, x: f64) -> f64 {
        let norm = (2.0 * 3.1415926535f64).powf(0.0 - 0.5) / sigma;
        norm.ln() + (0.0 - ((x - mu) / sigma).powi(2) / 2.0)

        // let norm = 1.0 / (2.0 * 3.1415926) / sigma;
        // let exp = (x - mu) / sigma;
        // let p = 0.0 - exp.powi(2);
        // norm * p.exp()

        // (1.0 / f64::sqrt(2.0 * f64::consts::PI * sigma))
        //     * f64::exp(-f64::powi(x - mu, 2) / (2.0 * sigma * sigma))
    }
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let b = Gaussian::_value_and_gradient([1.0f64, 1.2f64], [0.0f64]);
        // println!("{:?}", b);
        // println!("DONEON");
        // assert_eq!(1.0, b.0);
        let n = 100000;
        let mut rng = rand::rng();
        let vec: Vec<[f64; 1]> = (0..n).map(|_| rng.random()).collect();

        let mut b = 0.0;

        for i in 0..1000 {
            let k: f64 = vec
                .iter()
                .map(|f| Gaussian::_value_and_gradient([1.0f64, 1.2f64], *f).0)
                .sum();
            b += k;
        }
        println!("Done: {}", b);
        println!(
            "{:?}",
            Gaussian::_value_and_gradient([1.0f64, 0.0f64], [0.3])
        );
        assert_eq!(0.0, 1.0);
    }
}
