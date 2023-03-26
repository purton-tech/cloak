use crate::keyring;

pub async fn select() {
    let mut keyring = keyring::KeyRing::load();

    let accounts = keyring.list_service_accounts();
    println!("You have local keys for the following service accounts.");
    println!("Select an account to make it active.");

    for (index, account) in accounts.iter().enumerate() {
        println!("{}) {}", index + 1, account);
    }
    println!("Enter a number or (q) to quit.");

    loop {
        let mut user_input = String::new();
        std::io::stdin()
            .read_line(&mut user_input)
            .expect("Could not get input");
        let user_input = user_input.trim();

        if user_input == "q" {
            break;
        }

        if let Ok(number) = user_input.parse::<u32>() {
            if number <= keyring.accounts.len() as u32 && number > 0 {
                keyring.select_service_account(number - 1);
                keyring.save();
                break;
            }
            println!("Number out of range");
        } else {
            println!("Enter a number or (q) to quit.");
        }
    }
}
