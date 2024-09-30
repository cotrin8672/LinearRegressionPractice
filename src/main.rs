extern crate nalgebra as na;

use na::{SMatrix, SVector};
use plotly::common::Mode;
use plotly::{Plot, Scatter};
use rand::Rng;

fn initialize_weight<const N: usize>() -> SMatrix<f64, 1, N> {
    SVector::<f64, N>::zeros().map(|_| { rand::thread_rng().gen_range(-0.01..0.01) }).transpose()
}

fn initialize_bias() -> f64 {
    rand::thread_rng().gen_range(-0.01..0.01)
}

fn generate_random_data<const N: usize>() -> (SVector<f64, N>, SVector<f64, N>) {
    let m = rand::thread_rng().gen_range(-5.00..5.00);
    let c = rand::thread_rng().gen_range(0.00..5.00);
    let x_data = SVector::<f64, N>::from_fn(|i, _| { i as f64 });
    let y_data = x_data.map(|i| { i * m + c + rand::thread_rng().gen_range(-20.0..20.00) });

    (x_data, y_data)
}

fn main() {
    const NUM_POINTS: usize = 200;
    const PREDICT_POINTS: usize = 10;

    let (x_data, y_data) = generate_random_data::<NUM_POINTS>();
    let trace = Scatter::new(x_data.as_slice().to_vec(), y_data.as_slice().to_vec())
        .name("random plots")
        .mode(Mode::LinesMarkersText);
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.show()
}
