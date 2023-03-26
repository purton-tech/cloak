use crate::config::Config;
use cli_table::WithTitle;
use cli_table::{print_stdout, Table};

#[derive(Table)]
struct SecretRow {
    #[table(title = "Name")]
    name: String,
    #[table(title = "Value")]
    value: String,
}

pub async fn secrets(config: &Config) {
    let secrets = config.get_secrets().await;

    if let Some(secrets) = secrets {
        let mut table: Vec<SecretRow> = Default::default();
        for (name, value) in secrets.into_iter() {
            table.push(SecretRow { name, value })
        }
        print_stdout(table.with_title()).expect("Problem printing out the secrets");
    } else {
        println!("To connect to and see secrets attach a service account.");
        println!("Your options are...");
        println!("1. Use cloak import to import a service account key to your local system");
        println!("2. Set the ECDH_PRIVATE_KEY environment variable to secret key string");
        println!("3. Store a key in a PEM file. i.e. cloak.pem");
    }
}
