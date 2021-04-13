use std::io;
use rand::Rng;
use std::cmp::Ordering; 

fn main() {
    println!("errate die Zahl. Zwischen 1-120");
    
    let number = rand::thread_rng().gen_range(1, 121);
    
    loop {
        let mut test = String::new(); 
        io::stdin().read_line(&mut test).expect("Fehler");
        let test: u32 = test.trim().parse().expect("Gib bitte eine Zahl ein");
        match test.cmp(&number) {
            Ordering::Less => println!("Zu wenig"),
            Ordering::Greater => println!("Zu viel"),
            Ordering::Equal => {
                println!("Gewonnen!");
                break;
           },
        }
    }
}
