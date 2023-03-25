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
        print_stdout(table.with_title()).expect("Problem pring out the secrets");
    }
}
