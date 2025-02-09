use option_pricing::{ExerciseDecisionFn, PathParameters, PayoffFn, PricingModel};
mod test;

fn main() {
    // Basic parameters
    let params = PathParameters {
        spot_price: 100.0,
        time_to_maturity: 1.0,
        risk_free_rate: 0.05,
        volatility: 0.2,
        dividend_yield: 0.0,
        steps: 100,
    };
    // Create pricing model
    let model = PricingModel::new(params.clone()).with_path_dependency();

    // Example: European call option
    let call_payoff: PayoffFn =
        Box::new(|path: &[f64], _t| f64::max(path.last().unwrap() - 100.0, 0.0));

    // Example: Asian option
    let asian_payoff: PayoffFn = Box::new(|path: &[f64], _t| {
        let avg = path.iter().sum::<f64>() / path.len() as f64;
        f64::max(avg - 100.0, 0.0)
    });

    // Example: American put option
    let put_payoff: PayoffFn =
        Box::new(|path: &[f64], _t| f64::max(100.0 - path.last().unwrap(), 0.0));

    let american_exercise: ExerciseDecisionFn =
        Box::new(|_path: &[f64], immediate: f64, continuation: f64, _step| {
            immediate > continuation
        });

    // Price options
    let european_call = model.price(call_payoff, None);
    let asian_call = model.price(asian_payoff, None);
    let american_put = model.price(put_payoff, Some(american_exercise));

    println!("European Call Price: {:.4}", european_call.price);
    println!("Asian Call Price: {:.4}", asian_call.price);
    println!("American Put Price: {:.4}", american_put.price);
}
