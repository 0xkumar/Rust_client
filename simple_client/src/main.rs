use tungstenite::connect;
use url::Url;
use std::io::BufReader;
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::{Write,BufRead};
use std::vec;
use clap::Parser;

static BINANCE_WS_API: &str = "wss://fstream.binance.com";
static PRICES_FILE:&str = "prices.txt";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args{
    #[arg(short, long)]
    mode: String,

    #[arg(short, long, default_value_t = 1)]
    times: u64,
}

fn main(){
    let mut _btc_prices:Vec<f64> = vec![];
    let mut _total:f64 = 0.0;
    let mut _avg:f64 = 0.0;


    let args = Args::parse();

    // cache mode implementation
    if args.mode == "cache" && args.times > 0{
        let _mode1 = &args.mode;
        let time:u64 = args.times;
        _btc_prices = get_prices(time);
        _avg = get_average(&_btc_prices);

        write_prices_to_file(&_btc_prices);
        println!("Average BTC price after listening for {} seconds is :  {}",time,_avg);
    }


    //T'read' mode implementation
    if args.mode == "read"{
        let _mode2 = args.mode;

        //Read prices from the file
        _btc_prices = read_prices_from_file();
        println!("Prices read from the file are :  {:?}",_btc_prices);
    }


}

/*@dev: This function will connect to the binance websocket and get the prices of BTC
        for the given time in seconds.
 */
fn get_prices(time:u64) -> Vec<f64>{
        let mut btc_prices:Vec<f64> = vec![];

        let start_time = std::time::Instant::now();
        let binance_url =format!("{}/ws/btcusdt@trade",BINANCE_WS_API);
        let (mut socket, _response) = connect(Url::parse(&binance_url).unwrap()).expect("can't connect.");



        while start_time.elapsed().as_secs() < time {
            let msg = socket.read_message().expect("error reading message");
            let msg = match msg {
                tungstenite::Message::Text(s) => s,
                _=>{
                    panic!("Error getting text");
                }
            };

            let parsed_data:serde_json::Value = serde_json::from_str(&msg).expect("unable to parse message");
            let price1 = parsed_data["p"].as_str().unwrap();
            let price = String::from(price1);

            let num1:f64 = price.parse().expect("not a number");
            btc_prices.push(num1);
            thread::sleep(Duration::from_millis(100));

    }

    btc_prices
}

/* @dev: This function will write the prices to the file.
 */
fn write_prices_to_file(btc_prices:&Vec<f64>){
    let mut file = File::create(PRICES_FILE).expect("unable to crate file");

        for prices in btc_prices{
            writeln!(file,"{}",prices).expect("Unable to write to file");
        }
    }


/* @dev: This function will read the prices from the file.
    */
fn read_prices_from_file() -> Vec<f64>{
    let file = File::open(PRICES_FILE).expect("unable to open file");
    let reader = BufReader::new(file);
    let mut btc_prices: Vec<f64> = vec![];
    for line in reader.lines(){
        if let Ok(price_str) = line {
            if let Ok(price) = price_str.parse(){
                btc_prices.push(price);
            }
        }
    }
    btc_prices
}

/* @dev: This function will get the average of the prices.
 */
fn get_average(btc_prices:&Vec<f64>) -> f64{
    let mut total:f64 = 0.0;
    let mut _leng:f64 = 0.0;
    let mut _avg:f64 = 0.0;

    for i in btc_prices{
        total = total + i;
    }
    _leng = btc_prices.len() as f64;
    _avg = total/_leng;
    _avg
}
