





fn process_data_bad(data: Vec<String>) {
    for item in data {
        // .clone() 
        print_item(item.clone()); 
    }
}
// Zero-copy, much cheaper, and gas-efficient on Solana.
fn process_data_good(data: &Vec<String>) {
    for item in data.iter() {
        // not taking ownership
        print_item_ref(item); 
    }
}

fn print_item(s: String) { println!("{}", s); }
fn print_item_ref(s: &str) { println!("{}", s); }

fn main () {
    let my_data = vec!["Hello".to_string(), "World".to_string()];

    process_data_bad(my_data.clone()); // cloning to preserve ownership
    process_data_good(&my_data); // passing reference
}