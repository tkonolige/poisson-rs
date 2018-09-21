extern crate poisson;
use poisson::Type;

extern crate rand;
use rand::{Rng, FromEntropy};
use rand::rngs::SmallRng;
use rand::distributions::normal::StandardNormal;

extern crate sphere;

extern crate nalgebra as na;
pub type Vect = na::Vector2<f64>;

extern crate alga;
use self::alga::linear::FiniteDimVectorSpace;

extern crate num_traits;
use num_traits::Zero;

use helper::When::*;

mod helper;

#[test]
fn multiple_too_close_invalid() {
    let samples = 100;
    let relative_radius = 0.8;
    let prefiller = |radius| {
        let mut last = None::<Vect>;
        let mut rand = SmallRng::from_entropy();
        move |v| {
            if let Some(_) = v {
                if last == v {
                    None
                } else {
                    last = v;
                    let vec = sphere_uniform_point(&mut rand);
                    v.map(|v| v + vec * f64::rand(&mut rand) * radius)
                }
            } else {
                None
            }
        }
    };
    helper::test_with_samples_prefilled(samples, relative_radius, 20, Type::Normal, prefiller, Never);
}

pub fn sphere_uniform_point<R: Rng>(rng: &mut R) -> Vect {
    let mut result = Vect::zero();
    for c in 0..Vect::dimension() {
        result[c] = rng.sample(StandardNormal);
    }
    result.normalize()
}
