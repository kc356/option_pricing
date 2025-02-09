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
}
