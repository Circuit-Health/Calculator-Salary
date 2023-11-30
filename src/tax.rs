pub struct TaxRate {
    lower_limit: f64,              // Lower limit of the tax bracket
    upper_limit: Option<f64>,      // Upper limit of the tax bracket (optional)
    base_tax: f64,                 // Base tax amount for this bracket
    marginal_rate: f64,            // Marginal tax rate for income in this bracket
    superannuation_rate: f64,      // Superannuation rate for the year
}

// Function to calculate the total tax based on salary and year
pub fn calculate_tax(salary: f64, year: u32) -> f64 {
    let rates = get_tax_rates(year);
    let mut tax = 0.0;

    // Iterate through the tax rates to calculate the total tax
    for rate in rates {
        if salary > rate.lower_limit {
            let taxable_income = match rate.upper_limit {
                Some(upper) => salary.min(upper) - rate.lower_limit,
                None => salary - rate.lower_limit,
            };
            tax += rate.base_tax + (taxable_income * rate.marginal_rate);
        }
    }

    tax
}

// Function to get the tax rates for a specific year
fn get_tax_rates(year: u32) -> Vec<TaxRate> {
    match year {
        2023 => vec![
            TaxRate { lower_limit: 0.0, upper_limit: Some(18200.0), base_tax: 0.0, marginal_rate: 0.0, superannuation_rate: 11.5 },
            TaxRate { lower_limit: 18201.0, upper_limit: Some(45000.0), base_tax: 0.0, marginal_rate: 0.19, superannuation_rate: 11.5  },
            TaxRate { lower_limit: 45001.0, upper_limit: Some(120000.0), base_tax: 5092.0, marginal_rate: 0.325, superannuation_rate: 11.5  },
            TaxRate { lower_limit: 120001.0, upper_limit: Some(180000.0), base_tax: 29467.0, marginal_rate: 0.37, superannuation_rate: 11.5  },
            TaxRate { lower_limit: 180001.0, upper_limit: None, base_tax: 51667.0, marginal_rate: 0.45, superannuation_rate: 11.5  },
        ],
        2024 => vec![
            TaxRate { lower_limit: 0.0, upper_limit: Some(18200.0), base_tax: 0.0, marginal_rate: 0.0, superannuation_rate: 11.5  },
            TaxRate { lower_limit: 18201.0, upper_limit: Some(45000.0), base_tax: 0.0, marginal_rate: 0.19, superannuation_rate: 11.5  },
            TaxRate { lower_limit: 45001.0, upper_limit: Some(120000.0), base_tax: 5092.0, marginal_rate: 0.325, superannuation_rate: 11.5  },
            TaxRate { lower_limit: 120001.0, upper_limit: Some(180000.0), base_tax: 29467.0, marginal_rate: 0.37, superannuation_rate: 11.5  },
            TaxRate { lower_limit: 180001.0, upper_limit: None, base_tax: 51667.0, marginal_rate: 0.45, superannuation_rate: 11.5  },
        ],
        _ => vec![],  // Default case with empty vector if year not matched
    }
}

// Function to get the superannuation rate for a specific year
pub fn get_superannuation_rate(year: u32) -> f64 {
    match year {
        2023 => 11.5, // Example superannuation rate for 2023
        // Define superannuation rates for other years
        // ...
        _ => 0.0, // Default case
    }
}

