#[macro_use]
extern crate clap;
extern crate indicatif;
extern crate rand;
extern crate serde;
extern crate serde_pickle;


use rand::distributions::{Distribution, StandardNormal};
use rand::prelude::*;

use clap::{App, Arg, SubCommand};

const PATH_NUM_DEFAULT: usize = 1000;
const CORE_NUM_DEFAULT: usize = 1;

const START_X_DEFAULT: f64 = 0.0;
const START_Y_DEFAULT: f64 = 0.0;

const A_DEFAULT: f64 = 0.0;
const B_DEFAULT: f64 = 0.0;

const G_DEFAULT: (f64, f64, f64, f64) = (1.0, 0.0, 0.0, 1.0);

const STEP_SIZE: f64 = 0.001;
const STEPS_DEFAULT: usize = 1000;

pub fn additive_brusselator(
    a: f64,
    b: f64,
    start_x: f64,
    start_y: f64,
    dt: f64,
    g: &Vec<f64>,
    steps: usize,
) -> Vec<(f64, f64)> {
    let mut positions = Vec::with_capacity(steps);
    let mut rng = thread_rng();
    let sq_dt = dt.sqrt();
    positions.push((start_x, start_y));
    for _ in 0..steps {
        let (x, y) = positions.last().unwrap();
        let w0 = StandardNormal.sample(&mut rng);
        let w1 = StandardNormal.sample(&mut rng);
        positions.push((
            x + (1.0 - (b + 1.0) * x + a * x.powi(2) * y) * dt + (g[0] * w0 + g[1] * w1) * sq_dt,
            y + (b * x - a * x.powi(2) * y) * dt + (g[2] * w0 + g[3] * w1) * sq_dt,
        ))
    }
    positions
}

fn run_additive_brusselator(a:f64, b:f64, x:f64, y:f64, dt:f64, g: &Vec<f64>, steps: usize, cores: usize, n_paths: usize) -> Vec<Vec<(f64, f64)>> {
    vec![]
}

fn main() -> std::io::Result<()> {
    let matches = App::new("Brusselator")
        .author("Max Tyler <maxastyler@gmail.com>")
        .arg(
            Arg::with_name("OUTPUT")
                .short("o")
                .help("Sets the file to save the photon paths to")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("num_paths")
                .short("p")
                .help(&format!(
                    "The number of paths to simulate. Default is {}",
                    PATH_NUM_DEFAULT,
                ))
                .takes_value(true),
        )
        .arg(
            Arg::with_name("num_cores")
                .short("c")
                .help(&format!(
                    "The number of CPU cores to use. Default is {}",
                    CORE_NUM_DEFAULT
                ))
                .takes_value(true),
        )
        .arg(
            Arg::with_name("start_x")
                .short("x")
                .help(&format!(
                    "The starting x point. Default is {}",
                    START_X_DEFAULT,
                ))
                .takes_value(true)
        )
        .arg(
            Arg::with_name("start_y")
                .short("y")
                .help(&format!(
                    "The starting y point. Default is {}",
                        START_Y_DEFAULT,
                ))
                .takes_value(true)
        )
        .arg(
            Arg::with_name("a")
                .short("a")
                .help(&format!(
                    "The value for a. Default is {}",
                    A_DEFAULT,
                ))
                .takes_value(true)
        )
        .arg(
            Arg::with_name("b")
                .short("b")
                .help(&format!(
                    "The value for b. Default is {}",
                    B_DEFAULT,
                ))
                .takes_value(true)
        )
        .arg(
            Arg::with_name("noise_coefficient")
                .short("g")
                .help(&format!(
                    "The noise matrix g^{{ij}}. Default is {} {} {} {}",
                    G_DEFAULT.0, G_DEFAULT.1, G_DEFAULT.2, G_DEFAULT.3,
                ))
                .takes_value(true)
                .number_of_values(4),
        )
        .arg(
            Arg::with_name("step_size")
                .short("d")
                .help(&format!(
                    "The timestep size, dt. Default is {}",
                    STEP_SIZE
                ))
                .takes_value(true)
        )
        .arg(
            Arg::with_name("steps")
                .short("s")
                .help(&format!(
                    "The number of steps to take on each path. Default is {}",
                    STEPS_DEFAULT,
                ))
                .takes_value(true)
        )
        .get_matches();
    let g_vec = values_t!(matches.values_of("noise_coefficient"), f64).unwrap_or(vec![
        G_DEFAULT.0,
        G_DEFAULT.1,
        G_DEFAULT.2,
        G_DEFAULT.3,
    ]);
    run_additive_brusselator(
        value_t!(matches, "a", f64).unwrap_or(A_DEFAULT),
        value_t!(matches, "b", f64).unwrap_or(B_DEFAULT),
        value_t!(matches, "start_x", f64).unwrap_or(START_X_DEFAULT),
        value_t!(matches, "start_y", f64).unwrap_or(START_Y_DEFAULT),
        value_t!(matches, "step_size", f64).unwrap_or(STEP_SIZE),
        &g_vec,
        value_t!(matches, "steps", usize).unwrap_or(STEPS_DEFAULT),
        value_t!(matches, "num_cores", usize).unwrap_or(CORE_NUM_DEFAULT),
        value_t!(matches, "num_paths", usize).unwrap_or(PATH_NUM_DEFAULT),
        );
    Ok(())
}
