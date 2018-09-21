extern crate poisson;
use poisson::{Type, Builder, algorithm};

extern crate rand;
use rand::SeedableRng;
use rand::rngs::SmallRng;
use rand::FromEntropy;

extern crate sphere;

extern crate nalgebra as na;
pub type Vect = na::Vector2<f64>;

extern crate num_traits;

use std::iter::repeat;

use helper::When::*;

mod helper;

#[test]
fn adding_valid_start_works() {
    let samples = 100;
    let relative_radius = 0.8;
    let rand = SmallRng::from_entropy();
    let prefiller = |_| {
        let mut pre = Builder::<_, Vect>::with_samples(samples, relative_radius, Type::Normal)
            .build(rand.clone(), algorithm::Ebeida)
            .into_iter()
            .take(25);
        move |_| pre.next()
    };
    helper::test_with_samples_prefilled(samples, relative_radius, 100, Type::Normal, prefiller, Always);
}

#[test]
fn adding_valid_middle_works() {
    let samples = 100;
    let relative_radius = 0.8;
    let rand = SmallRng::from_entropy();
    let prefiller = |_| {
        let prefiller = Builder::<_, Vect>::with_samples(samples, relative_radius, Type::Normal)
            .build(rand.clone(), algorithm::Ebeida);
        let mut pre = repeat(None)
            .take(25)
            .chain(prefiller
                .into_iter()
                .take(25)
                .map(Some));
        move |_| pre.next().and_then(|s| s)
    };

    helper::test_with_samples_prefilled(samples, relative_radius, 100, Type::Normal, prefiller, Sometimes);
}

#[test]
fn adding_to_edges_start_works() {
    let samples = 100;
    let relative_radius = 0.8;
    let prefiller = [
        Vect::new(0.0, 0.0), Vect::new(0.0, 0.5),
        Vect::new(0.0, 1.0), Vect::new(0.5, 0.0),
        Vect::new(1.0, 0.0), Vect::new(0.5, 1.0),
        Vect::new(1.0, 0.5), Vect::new(1.0, 1.0),
        ];
    let prefiller = |_| {
        let mut pre = prefiller.iter().cloned().map(Some as fn(_) -> _);
        move |_| pre.next().and_then(|s| s)
    };
    helper::test_with_samples_prefilled(samples, relative_radius, 100, Type::Normal, prefiller, Always);
}

#[test]
fn adding_to_outside_of_edges_start_works() {
    let samples = 100;
    let relative_radius = 0.8;
    let prefiller = [
        Vect::new(-0.1, -0.1), Vect::new(-0.1, 0.5),
        Vect::new(-0.1, 1.1), Vect::new(0.5, -0.1),
        Vect::new(1.1, -0.1), Vect::new(0.5, 1.1),
        Vect::new(1.1, 0.5), Vect::new(1.1, 1.1),
        ];
    let prefiller = |_| {
        let mut pre = prefiller.iter().cloned().map(Some as fn(_) -> _);
        move |_| pre.next().and_then(|s| s)
    };
    helper::test_with_samples_prefilled(samples, relative_radius, 100, Type::Normal, prefiller, Always);
}

#[test]
fn completely_filled_works() {
    let samples = 100;
    let relative_radius = 0.8;
    let rand = SmallRng::from_entropy();
    let prefiller = |_| {
        let mut pre = Builder::<_, Vect>::with_samples(samples, relative_radius, Type::Normal)
            .build(rand.clone(), algorithm::Ebeida)
            .into_iter();
        move |_| pre.next()
    };
    helper::test_with_samples_prefilled(samples, relative_radius, 100, Type::Normal, prefiller, Always);
}
