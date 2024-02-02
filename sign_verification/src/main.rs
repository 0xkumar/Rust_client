use ring::rand::SystemRandom;
use ring::signature::KeyPair;
use ring::signature::UnparsedPublicKey;
use ring::signature::EcdsaKeyPair;
use ring::signature::ECDSA_P256_SHA256_ASN1;
use ring::signature::ECDSA_P256_SHA256_ASN1_SIGNING;

use tungstenite::connect;
use url::Url;
use std::time::Instant;
use std::thread;
use std::sync::{mpsc,Arc,Mutex};


static BINANCE_WS_API: &str = "wss://fstream.binance.com";
fn main() {

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
            //Thread creation
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
                            panic!("\n Error getting text Try again \n");
                        }
                    };
            
                    let parsed_data:serde_json::Value = serde_json::from_str(&msg).expect("unable to parse message");
                    let price1 = parsed_data["p"].as_str().unwrap();
                    let price = String::from(price1);
            
                    let num1:f64 = price.parse().expect("unable to parse price");
                    btc_prices.push(num1);
                }

                for i in &btc_prices{
                    avg = avg + i;
                }
                //Calculate the average of the prices.
                _total_avg = avg/btc_prices.len() as f64;

                //Encrypt the total_avg using ECDSA algorithm.
                let rand= SystemRandom::new();
                let pkcs8_bytes = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_ASN1_SIGNING,&rand).unwrap();

                //@dev: The key pair is generated and the public key is sent to the other party.
                //@ notice --> If you got error here remove the third argument '&rand'. 

                let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_ASN1_SIGNING,pkcs8_bytes.as_ref(),&rand).unwrap();
                let messae:[u8;8] = _total_avg.to_be_bytes();
                let sig = key_pair.sign(&rand,&messae).unwrap();
                let peer_public_key_bytes = key_pair.public_key().as_ref();

                let message = (_total_avg,peer_public_key_bytes.to_vec(),messae.to_vec(),sig);

                //This is the sending part to the reveiver which is 'rx'.
                let tx = tx.lock().unwrap();
                tx.send(message).unwrap();
            });
            
            handle
           
        }
    
        ).collect();

        for handle in handles{
            handle.join().unwrap();
        }

        //The receiver will receive the message and verify the signature.
        for i in rx{
            let (total_avg,peer_public_key_bytes,message,sig) = i;
            let const_message : &[u8] = &message;
           
           //The public key is parsed and the signature is verified.
            let peer_public_key = UnparsedPublicKey::new(&ECDSA_P256_SHA256_ASN1, peer_public_key_bytes);
            peer_public_key.verify(const_message, sig.as_ref()).expect("signature is invalid");
            
            total_btc_prices.push(total_avg);


            if total_btc_prices.len() == 5 {
                break;
            }
            
        }

        //Calculate the final average of the prices and print it.
        for i in &total_btc_prices{
            final_btc_avg = final_btc_avg + i;
        }
        println!("\nThe final average BTC price is : {:?}\n", final_btc_avg/5.00);    
}
