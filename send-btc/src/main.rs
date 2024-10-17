use std::io;
use  rand::Rng;

fn send_bitcoin() {
    let users = vec!["Berba", "Jerry", "Leonardo", "Botticelli"]; 

    println!("Who are you sending to?\n");

    for user in &users {
        print!("{} ", user);
    }
    println!("\nSelect one of your contacts to send BTC to\n");

    let mut selection = String::new();
    
    io::stdin().read_line(&mut selection);

    if(users.contains(&selection.trim())) {
        println!("How many bitcoins you want to send to {}?", selection.trim());

        let mut amount = String::new();

        io::stdin().read_line(&mut amount);

        println!("You're sending {} btc to {}!", amount.trim(), selection.trim());
    } else {
        println!("{} is not in your contact list", selection.trim());
    }
}

fn receive_bitcoin() {
    let number = rand::thread_rng().gen_range(1, 10);

    println!("You received {} btc\n", number);
}

fn console_exit() {
    println!("Invalid Input!\n")
}

fn console() {
    println!("\nWelcome to the btc wallet!\n"); 
    
    println!("Do you want to (s) send or (r) receive bitcion?\n");

    let mut command = String::new(); 

    io::stdin().read_line(&mut command);

    if(command.trim().eq("s")) {
        send_bitcoin();
    } else if(command.trim().eq("r")) {
        receive_bitcoin();
    } else {
        console_exit();
    }
}

fn main() {
    console();
}