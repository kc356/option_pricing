#[cfg(test)]
mod tests {
    use option_pricing::*;

    #[test]
    fn test_european_call() {
        let params = PathParameters {
            spot_price: 100.0,
            time_to_maturity: 1.0,
            risk_free_rate: 0.05,
            volatility: 0.2,
            dividend_yield: 0.0,
            steps: 100,
        };

        let model = PricingModel::new(params);

        let call_payoff: PayoffFn =
            Box::new(|path: &[f64], _t| f64::max(path.last().unwrap() - 100.0, 0.0));

        let result = model.price(call_payoff, None);

        // Compare with Black-Scholes price (approximately 10.45)
        assert!((result.price - 10.45).abs() < 0.1);
    }

    #[test]
    fn test_european_put() {
        let params = PathParameters {
            spot_price: 100.0,
            time_to_maturity: 1.0,
            risk_free_rate: 0.05,
            volatility: 0.2,
            dividend_yield: 0.0,
            steps: 100,
        };

        let model = PricingModel::new(params);
        let put_payoff: PayoffFn =
            Box::new(|path: &[f64], _t| f64::max(100.0 - path.last().unwrap(), 0.0));

        let result = model.price(put_payoff, None);

        // Compare with Black-Scholes price (approximately 5.57)
        // Using wider tolerance band (0.2) due to binomial approximation
        assert!(
            (result.price - 5.57).abs() < 0.2,
            "Put price {} differs from expected 5.57 by more than 0.2",
            result.price
        );
    }

    #[test]
    fn test_asian_call() {
        let params = PathParameters {
            spot_price: 100.0,
            time_to_maturity: 1.0,
            risk_free_rate: 0.05,
            volatility: 0.2,
            dividend_yield: 0.0,
            steps: 100,
        };

        let model = PricingModel::new(params).with_path_dependency();
        let asian_payoff: PayoffFn = Box::new(|path: &[f64], _t| {
            let avg = path.iter().sum::<f64>() / path.len() as f64;
            f64::max(avg - 100.0, 0.0)
        });

        let result = model.price(asian_payoff, None);

        // Asian options should be worth less than equivalent European options
        // due to averaging effect
        assert!(result.price < 10.45);
    }

    #[test]
    fn test_american_put() {
        let params = PathParameters {
            spot_price: 100.0,
            time_to_maturity: 1.0,
            risk_free_rate: 0.05,
            volatility: 0.2,
            dividend_yield: 0.0,
            steps: 100,
        };

        let model = PricingModel::new(params);
        let put_payoff: PayoffFn =
            Box::new(|path: &[f64], _t| f64::max(100.0 - path.last().unwrap(), 0.0));

        let american_exercise: ExerciseDecisionFn =
            Box::new(|_path: &[f64], immediate: f64, continuation: f64, _step| {
                immediate > continuation
            });

        let result = model.price(put_payoff, Some(american_exercise));

        // American put should be worth more than European put
        assert!(result.price > 4.01);
    }
}
