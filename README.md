## Task1 - simple_client

#### ABOUT

##### Simple Client runs two commads 
simple --mode=cache --times=10 <br>
simple --mode=read

=> If the user gives command 1 as input the client listen the USD price of BTC for given number
of times and stores the prices in a text file.
=> If the user gives the commad 2 as input the client fetches the data from the text file
and displays on the terminal.
#### DEMO
```bash
  cargo run -- --simple --mode=cache --times=10
```
```bash
  cargo run -- --simple --mode=read
```

## Task2 - multi_client

#### ABOUT

In Multi client 5 client processes read values from the websockets for 10 seconds 
and computes the average and sends the average to the aggregator process.
Then the aggregator process waits for the average values from all the 5 clients. 
After getting the values it computes the average again and displays it on the terminal.

#### DEMO
```bash
  cargo run
```

## Task3 - sign_verification
#### ABOUT
In this sign_verification clients sign their average values and sends the value and signatures
to the aggregator upon receiving the signatures the aggregator verfies the signatures.For this 
verification process ECDSA have been used.
```bash
  cargo run
```
