# Option Pricing

This project implements option pricing models in Rust. It includes functionality for pricing European, Asian, and American options using a binomial tree model.

## Project Structure

- `src/lib.rs`: Contains the core logic for the pricing models.
- `src/main.rs`: Example usage of the pricing models.
- `src/test.rs`: Unit tests for the pricing models.

## Getting Started

### Prerequisites

- Rust (latest stable version)

### Installation

Clone the repository:

```sh
git clone https://github.com/yourusername/option_pricing.git
cd option_pricing
```

### Building the Project

To build the project, run:

```sh
cargo build
```

### Running Tests

To run the tests, use:

```sh
cargo test
```

## Usage

You can use the `PricingModel` to price different types of options. Here is an example for a European call option:

```rust
use option_pricing::*;

fn main() {
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

    println!("Option price: {}", result.price);
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.