use tungstenite::connect;
use url::Url;
use std::time::Instant;
use std::thread;
use std::sync::{mpsc,Arc,Mutex};

static BINANCE_WS_API: &str = "wss://fstream.binance.com";

fn main(){
    let mut total_btc_prices:Vec<f64> = vec![];
    let mut final_btc_avg:f64 = 0.0;
    let start_time = Instant::now();
    let (tx,rx) = mpsc::channel();
    let tx = Arc::new(Mutex::new(tx));

    /*@dev: The handles will be used to store the threads that will be created.
            Loop will run for 5 times and each time a thread will be created.
     */
    let handles:Vec<_> = (0..5)
        .map(|_i|{
            let tx = Arc::clone(&tx);
            let handle = thread::spawn(move||{
                let mut btc_prices:Vec<f64> = vec![];
                let mut _total_avg:f64 = 0.0;
                let mut avg:f64 = 0.0;
                let binance_url =format!("{}/ws/btcusdt@aggTrade",BINANCE_WS_API);
                let (mut socket, _response) = connect(Url::parse(&binance_url).unwrap()).expect("can't connect.");
                
                // The loop will run for 10 seconds and get the prices of BTC. 
                while start_time.elapsed().as_secs() < 10{
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
                }
                for i in &btc_prices{
                    avg = avg + i;
                }
                //Calculate the average of the prices.
                _total_avg = avg/btc_prices.len() as f64;
                let  tx = tx.lock().unwrap();

                //Send the average to the main thread.
                tx.send(_total_avg).unwrap();
            });

            handle
           
        }
        //Collect the handles in a vector.
        ).collect();

        //Join the threads.
        for handle in handles{
            handle.join().unwrap();
        }

        for i in rx{
            total_btc_prices.push(i);

            if total_btc_prices.len() == 5{
                break;
            }
            
        }
        //Calculate the final average of the prices.
        for i in &total_btc_prices{
            final_btc_avg = final_btc_avg + i;
        }   
        println!("The Final average BTC price is : {:?}", final_btc_avg/5.00);
    
}
