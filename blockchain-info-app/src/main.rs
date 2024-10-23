#[macro_use]
extern crate serde_derive;

mod blockchain_status;
mod blockchain_info;
mod blockchain_address;
mod blockchain_transaction;

use {
    crate::blockchain_status::BlockchainStatus,
    crate::blockchain_address::BlockchainAddress,
    crate::blockchain_transaction::BlockchainTransaction,
    dotenv,
    std::{io, thread, time},
};


fn blockchain_info_app(address: &str) {
    let blockchain_status: BlockchainStatus = blockchain_info::blockchain_status_request();
    println!("\n\nQuerying {} - chain: {}\n\n", &blockchain_status.blockbook.coin, &blockchain_status.backend.chain);

    let blockchain_address: BlockchainAddress = blockchain_info::blockchain_address_request(address);
    println!("\nChecking Ethereum address: {}\n", &blockchain_address.address);

    let sleep_time = time::Duration::from_millis(2500);
    thread::sleep(sleep_time);

    println!("\n There are {} transactions made with this address\n", &blockchain_address.txids.len());

    println!("\n Do you want to inspect these transactions? y/n?\n");

    let mut answer = String::new();
    io::stdin().read_line(&mut answer);

    if answer.trim().eq("y") {
        println!("\nLooking into the following txs: "); 
        thread::sleep(sleep_time);

        println!("{:#?}", &blockchain_address.txids);

        let mut balance: i64 = 0;

        for tx_id in &blockchain_address.txids {
            let mut sent: i64 = 0;
            let mut received: i64 = 0;
            let mut fees_paid: i64 = 0;

            let blockchain_transaction: BlockchainTransaction = blockchain_info::blockchain_transaction_request(tx_id);
            let match_address = String::from(address);
            for tx in &blockchain_transaction.vin {
                if tx.addresses.contains(&match_address) {
                    for transaction in &blockchain_transaction.vout {
                        sent += transaction.value.parse::<i64>().unwrap();
                    }
                    fees_paid += &blockchain_transaction.fees.parse::<i64>().unwrap();
                }
            }
            for tx in &blockchain_transaction.vout {
                if tx.addresses.contains(&match_address) {
                    received += tx.value.parse::<i64>().unwrap();
                }
            }

            balance += &received - &sent - &fees_paid;

            println!("-----------------------------------------------------");
            println!("TX ID:      {}", &blockchain_transaction.txid);
            println!("ETH IN:     {}", &received);
            println!("ETH OUT:    {}", &sent);
            println!("FEES_PAID:  {}", &fees_paid);
            println!("BALANCE:    {}", &balance);
            println!("-----------------------------------------------------");
        }

    }


}

fn main() {
    let entered_address = dotenv::var("ETH_ADDRESS").expect("Variable not found");
    blockchain_info_app(&entered_address);
}
