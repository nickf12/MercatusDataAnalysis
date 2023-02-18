mod lib;
use crate::lib::*;
use std::env;

#[tokio::main]
async fn main() {
    // CLI info:
    //First argument refers to the ticker. Second argument refers to the initial investment
    let args: Vec<String> = env::args().collect();
    let ticker =  &args[1].as_str();
    let initial_investment = &args[2].as_str().parse::<f32>().unwrap();

    // Run backtesting on last 6 months historical data
    // This backtesting functionality calculate gains for the best scenario. This means, buy at daily low and seel at daily high
    backtesting(ticker, initial_investment).await;

    // TODO, improve backtesting on prompted time frame from the user

    // TODO, improve backtesting calculation as in a real world is hard to buy at daily low and sell at daily high. Some logic for entry and exit price should be implemented 
}