pub type PayoffFn = Box<dyn Fn(&[f64], f64) -> f64>;
pub type ExerciseDecisionFn = Box<dyn Fn(&[f64], f64, f64, usize) -> bool>;

#[derive(Debug, Clone)]
pub struct PathParameters {
    pub spot_price: f64,
    pub time_to_maturity: f64,
    pub risk_free_rate: f64,
    pub volatility: f64,
    pub dividend_yield: f64,
    pub steps: u32,
}
#[derive(Debug, Clone)]
pub struct PricingModel {
    params: PathParameters,
    path_dependent: bool,
    store_paths: bool,
}

#[derive(Debug)]
pub struct PricingResult {
    pub price: f64,
    pub paths: Option<Vec<Vec<f64>>>,
    pub early_exercise_boundary: Option<Vec<f64>>,
}
