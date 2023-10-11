use crate::model::account::{AccountModel, DepartmentModel};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AccountDto {
    pub id: i32,
    pub name: String,
    pub account: String,
    // pub password: String,
    pub department_id: i32,
    pub department: String,
    pub steps: Vec<i32>,
}

impl AccountDto {
    pub fn from(account: AccountModel, department: DepartmentModel) -> AccountDto {
        Self {
            id: account.id,
            name: account.name,
            account: account.account,
            department_id: department.id,
            department: department.name,
            steps: department.steps,
        }
    }
}
