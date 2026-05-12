use std::collections::HashMap;
use std::io;

fn main() {
    let mut bills: HashMap<String, f64> = HashMap::new();

    loop {
        println!("\n=== BILL MANAGER ===");
        println!("1. Add Bill");
        println!("2. View Bills");
        println!("3. Remove Bill");
        println!("4. Edit Bill");
        println!("5. Exit");

        let choice = input("Choose an option: ");

        match choice.trim() {
            "1" => add_bill(&mut bills),
            "2" => view_bills(&bills),
            "3" => remove_bill(&mut bills),
            "4" => edit_bill(&mut bills),
            "5" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice"),
        }
    }
}

// ---------------- ADD BILL ----------------
fn add_bill(bills: &mut HashMap<String, f64>) {
    let name = input("Enter bill name: ");
    let amount = input("Enter amount owed: ");

    let amount: f64 = match amount.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid amount");
            return;
        }
    };

    bills.insert(name.trim().to_string(), amount);

    println!("Bill added successfully!");
}

// ---------------- VIEW BILLS ----------------
fn view_bills(bills: &HashMap<String, f64>) {
    if bills.is_empty() {
        println!("No bills found.");
        return;
    }

    println!("\n--- Bills ---");

    for (name, amount) in bills {
        println!("{} -> ${}", name, amount);
    }
}

// ---------------- REMOVE BILL ----------------
fn remove_bill(bills: &mut HashMap<String, f64>) {
    let name = input("Enter bill name to remove: ");

    if bills.remove(name.trim()).is_some() {
        println!("Bill removed.");
    } else {
        println!("Bill not found.");
    }
}

// ---------------- EDIT BILL ----------------
fn edit_bill(bills: &mut HashMap<String, f64>) {
    let name = input("Enter bill name to edit: ");

    if !bills.contains_key(name.trim()) {
        println!("Bill not found.");
        return;
    }

    let new_amount = input("Enter new amount (or type BACK): ");

    if new_amount.trim().to_uppercase() == "BACK" {
        println!("Edit cancelled.");
        return;
    }

    let amount: f64 = match new_amount.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid amount");
            return;
        }
    };

    bills.insert(name.trim().to_string(), amount);

    println!("Bill updated successfully!");
}

// ---------------- INPUT FUNCTION ----------------
fn input(message: &str) -> String {
    let mut value = String::new();

    println!("{}", message);

    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read input");

    value
}
