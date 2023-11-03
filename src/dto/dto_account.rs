use crate::model::account::AccountModel;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AccountDto {
    pub id: i32,
    pub name: String,
    pub account: String,
}

impl AccountDto {
    pub fn from(account: AccountModel) -> AccountDto {
        Self {
            id: account.id,
            name: account.name,
            account: account.account,
        }
    }
}
