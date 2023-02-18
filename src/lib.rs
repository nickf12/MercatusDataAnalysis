use yahoo_finance::{ history, Timestamped }; 

// DataPipeline struct
// Used to store the data for backtesting purposes
// TODO Implement an override of the constructor, so every time new record is created investment_amount will be updated
#[derive(Debug)]
pub struct DataPipeline {
    pub ticker: String,
    pub day: String,
    pub price_difference: f64,
    pub daily_low: f64,
    pub daily_high: f64,
    pub daily_gain: f32,
}

impl DataPipeline {
    // Print data
    pub fn show_data(self: &Self) {
        println!("The Pair {} on day {} had a low price of ${} and an high price of ${}, so the daily price difference was %{}. The daily gain is ${}", self.ticker, self.day, self.daily_low, self.daily_high, self.price_difference, self.daily_gain);
    }
    
    
}

/// Retrieves (at most) 6 months worth of OCLHV data for a symbol
/// ending on the last market close.
pub async fn backtesting(ticker:&str, initial_investment: &f32) {
    let mut profit = 0.;
    match history::retrieve(ticker).await {
       Err(e) => println!("Failed to call Yahoo: {:?}", e),
       Ok(data) => 
          for bar in &data {
             // println!("On {} {} closed at ${:.2}", bar.datetime().format("%b %e %Y"), ticker, bar.close);
             // println!("On {} {} high at ${:.7} and low at ${:.7}", bar.datetime().format("%b %e %Y"), ticker, bar.high, bar.low);
             // calculate price difference
             // println!("Daily price difference is %{:.7}", calculate_percentage(bar.low, bar.high));
             let _data = DataPipeline {
                ticker: format!("{}", ticker),
                day: format!("{}",bar.datetime().format("%b %e %Y")),
                price_difference: calculate_percentage(bar.low, bar.high),
                daily_low: bar.low,
                daily_high: bar.high,
                daily_gain: initial_investment / 100. * (calculate_percentage(bar.low, bar.high) as f32),
             };
             profit += initial_investment / 100. * (calculate_percentage(bar.low, bar.high) as f32);
             _data.show_data();
        }
    }
    println!("Your profit after 6 months should be ${}", profit);
}

// Calculate price difference percentage
pub fn calculate_percentage(low:f64, high:f64) -> f64 {
    return ((high - low) / low) * 100.;
}