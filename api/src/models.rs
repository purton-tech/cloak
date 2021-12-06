use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Default, Debug)]
pub struct NewVault {
    #[validate(length(min = 1, message = "The name is mandatory"))]
    pub name: String,
}
