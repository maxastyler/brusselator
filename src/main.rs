#[macro_use]
extern crate clap;
extern crate indicatif;
extern crate rand;
extern crate serde;
extern crate serde_pickle;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;

use indicatif::{ProgressBar, ProgressStyle};
use rand::distributions::{Distribution, StandardNormal};
use rand::prelude::*;

use std::fs::File;

use clap::{App, Arg, SubCommand};

const PATH_NUM_DEFAULT: usize = 1000;
const CORE_NUM_DEFAULT: usize = 1;

const START_X_DEFAULT: f64 = 0.0;
const START_Y_DEFAULT: f64 = 0.0;

const A_DEFAULT: f64 = 1.0;
const B_DEFAULT: f64 = 2.0;

const G_DEFAULT: (f64, f64, f64, f64) = (0.0, 0.0, 0.0, 0.0);

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

pub fn multiplicative_brusselator(
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

fn run_additive_brusselator(
    a: f64,
    b: f64,
    x: f64,
    y: f64,
    dt: f64,
    g: Vec<f64>,
    steps: usize,
    cores: usize,
    n_paths: usize,
    value_step: Option<usize>,
    skip: Option<usize>
) -> Vec<Vec<(f64, f64)>> {
    let path_count = Arc::new(AtomicUsize::new(0));
    let mut thread_handles = vec![];
    let (tx, rx) = mpsc::channel();
    for _ in 0..cores - 1 {
        let path_count_clone = path_count.clone();
        let tx_clone = tx.clone();
        let g_clone = g.clone();
        thread_handles.push(thread::spawn(move || loop {
            if path_count_clone.load(Ordering::SeqCst) < n_paths {
                path_count_clone.fetch_add(1, Ordering::SeqCst);
                tx_clone
                    .send(
                        additive_brusselator(a, b, x, y, dt, &g_clone, steps)
                            .iter()
                            .skip(skip.unwrap_or(0))
                            .step_by(value_step.unwrap_or(1))
                            .map(|&x| x)
                            .collect(),
                    )
                    .unwrap();
            } else {
                break;
            }
        }))
    }
    let bar = ProgressBar::new(n_paths as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "[{elapsed_precise}][{eta_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
            )
            .progress_chars("##-"),
    );
    let path_count_clone = path_count.clone();
    let tx_clone = tx.clone();
    loop {
        if path_count_clone.load(Ordering::SeqCst) < n_paths {
            path_count_clone.fetch_add(1, Ordering::SeqCst);
            tx_clone
                .send(
                    additive_brusselator(a, b, x, y, dt, &g, steps)
                        .iter()
                        .skip(skip.unwrap_or(0))
                        .step_by(value_step.unwrap_or(1))
                        .map(|&x| x)
                        .collect(),
                )
                .unwrap();
            bar.set_position(path_count_clone.load(Ordering::SeqCst) as u64);
        } else {
            break;
        }
    }
    bar.finish();
    for handle in thread_handles {
        handle.join().unwrap();
    }
    rx.try_iter().collect()
}

fn main() -> std::io::Result<()> {
    let matches = App::new("Brusselator")
        .author("Max Tyler <maxastyler@gmail.com>")
        .arg(
            Arg::with_name("OUTPUT")
                .short("o")
                .help("Sets the file to save the photon paths to")
                .takes_value(true),
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
                .takes_value(true),
        )
        .arg(
            Arg::with_name("start_y")
                .short("y")
                .help(&format!(
                    "The starting y point. Default is {}",
                    START_Y_DEFAULT,
                ))
                .takes_value(true),
        )
        .arg(
            Arg::with_name("a")
                .short("a")
                .help(&format!("The value for a. Default is {}", A_DEFAULT,))
                .takes_value(true),
        )
        .arg(
            Arg::with_name("b")
                .short("b")
                .help(&format!("The value for b. Default is {}", B_DEFAULT,))
                .takes_value(true),
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
                .help(&format!("The timestep size, dt. Default is {}", STEP_SIZE))
                .takes_value(true),
        )
        .arg(
            Arg::with_name("steps")
                .short("s")
                .help(&format!(
                    "The number of steps to take on each path. Default is {}",
                    STEPS_DEFAULT,
                ))
                .takes_value(true),
        )
        .arg(
            Arg::with_name("no_write")
                .short("w")
                .help("Don't write out to a file"),
        )
        .arg(
            Arg::with_name("nth")
                .short("k")
                .help("Only save every nth entry")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("skip")
                .short("j")
                .help("Skip the first n values")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("estimate_size")
                .short("e")
                .help("Give a rough file size, then quit"),
        )
        .get_matches();
    let g_vec = values_t!(matches.values_of("noise_coefficient"), f64).unwrap_or(vec![
        G_DEFAULT.0,
        G_DEFAULT.1,
        G_DEFAULT.2,
        G_DEFAULT.3,
    ]);
    let a = value_t!(matches, "a", f64).unwrap_or(A_DEFAULT);
    let b = value_t!(matches, "b", f64).unwrap_or(B_DEFAULT);
    let x = value_t!(matches, "start_x", f64).unwrap_or(START_X_DEFAULT);
    let y = value_t!(matches, "start_y", f64).unwrap_or(START_Y_DEFAULT);
    let step = value_t!(matches, "step_size", f64).unwrap_or(STEP_SIZE);
    let n_step = value_t!(matches, "steps", usize).unwrap_or(STEPS_DEFAULT);
    let cores = value_t!(matches, "num_cores", usize).unwrap_or(CORE_NUM_DEFAULT);
    let paths = value_t!(matches, "num_paths", usize).unwrap_or(PATH_NUM_DEFAULT);
    let value_step: Option<usize> = value_t!(matches, "nth", usize).ok();
    let skip: Option<usize> = value_t!(matches, "skip", usize).ok();
    let path_name = format!(
        "a_{}_b_{}_x_{}_y_{}_g_{}_{}_{}_{}_dt_{}.brus",
        a, b, x, y, g_vec[0], g_vec[1], g_vec[2], g_vec[3], step
    );
    let output_name = matches.value_of("OUTPUT").unwrap_or(&path_name);
    if matches.is_present("estimate_size") {
        println!(
            "The estimated size of the file is: {}MB",
            16.0 * (paths as f64) * ((n_step-skip.unwrap_or(0)) as f64) / (1024f64.powi(2) * value_step.unwrap_or(1) as f64)
        )
    } else {
        if !matches.is_present("no_write") {
            let mut file = File::create(output_name)?;
            serde_pickle::ser::to_writer(
                &mut file,
                &run_additive_brusselator(a, b, x, y, step, g_vec, n_step, cores, paths, value_step, skip),
                true,
            )
            .unwrap();
        } else {
            let _ = run_additive_brusselator(a, b, x, y, step, g_vec, n_step, cores, paths, value_step, skip);
        }
    }
    Ok(())

}
