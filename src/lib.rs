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

impl PricingModel {
    pub fn new(params: PathParameters) -> Self {
        PricingModel {
            params,
            path_dependent: false,
            store_paths: false,
        }
    }

    pub fn with_path_dependency(mut self) -> Self {
        self.path_dependent = true;
        self.store_paths = true;
        self
    }

    pub fn with_stored_paths(mut self) -> Self {
        self.store_paths = true;
        self
    }

    pub fn price(
        &self,
        payoff: PayoffFn,
        exercise_decision: Option<ExerciseDecisionFn>,
    ) -> PricingResult {
        let dt = self.params.time_to_maturity / self.params.steps as f64;
        let u = f64::exp(self.params.volatility * f64::sqrt(dt));
        let d = 1.0 / u;
        let r_adjusted = self.params.risk_free_rate - self.params.dividend_yield;
        let p = (f64::exp(r_adjusted * dt) - d) / (u - d);
        let discount = f64::exp(-self.params.risk_free_rate * dt);

        // Initialize price tree and path storage
        let mut price_tree =
            vec![vec![0.0; (self.params.steps + 1) as usize]; (self.params.steps + 1) as usize];
        let mut paths = if self.store_paths {
            Some(Vec::with_capacity((self.params.steps + 1) as usize))
        } else {
            None
        };
        let mut early_exercise = if exercise_decision.is_some() {
            Some(vec![f64::NAN; (self.params.steps + 1) as usize])
        } else {
            None
        };

        // Build stock price tree
        for i in 0..=self.params.steps {
            let mut row = Vec::with_capacity((i + 1) as usize);
            for j in 0..=i {
                let price = self.params.spot_price * u.powi(j as i32) * d.powi((i - j) as i32);
                price_tree[i as usize][j as usize] = price;
                if self.store_paths {
                    row.push(price);
                }
            }
            if self.store_paths {
                paths.as_mut().unwrap().push(row);
            }
        }

        // Terminal payoffs
        let mut values = vec![0.0; (self.params.steps + 1) as usize];
        for j in 0..=self.params.steps {
            let path = if self.path_dependent {
                &price_tree[..=self.params.steps as usize]
                    .iter()
                    .map(|row| row[j.min((row.len() - 1) as u32) as usize])
                    .collect::<Vec<_>>()
            } else {
                &vec![price_tree[self.params.steps as usize][j as usize]]
            };
            values[j as usize] = payoff(path, self.params.time_to_maturity);
        }

        // Backward induction
        for i in (0..self.params.steps).rev() {
            for j in 0..=i {
                let continuation =
                    discount * (p * values[j as usize + 1] + (1.0 - p) * values[j as usize]);

                if let Some(ref exercise_fn) = exercise_decision {
                    let current_price = price_tree[i as usize][j as usize];
                    let path = if self.path_dependent {
                        &price_tree[..=i as usize]
                            .iter()
                            .map(|row| row[j.min((row.len() - 1) as u32) as usize])
                            .collect::<Vec<_>>()
                    } else {
                        &vec![current_price]
                    };
                    let immediate_exercise = payoff(path, i as f64 * dt);

                    if exercise_fn(path, immediate_exercise, continuation, i as usize) {
                        values[j as usize] = immediate_exercise;
                        if let Some(ref mut boundary) = early_exercise {
                            boundary[i as usize] = current_price;
                        }
                    } else {
                        values[j as usize] = continuation;
                    }
                } else {
                    values[j as usize] = continuation;
                }
            }
        }

        PricingResult {
            price: values[0],
            paths,
            early_exercise_boundary: early_exercise,
        }
    }
}
