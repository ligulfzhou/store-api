use crate::model::customer::CustomerModel;

#[derive(Debug, Deserialize, Serialize)]
pub struct CustomerDto {
    pub id: i32,
    pub customer_no: String,
    pub name: String,
    pub address: String,
    pub phone: String,
    pub notes: String,
}

impl CustomerDto {
    pub fn from(customer: CustomerModel) -> CustomerDto {
        Self {
            id: customer.id,
            customer_no: customer.customer_no,
            name: customer.name,
            address: customer.address,
            phone: customer.phone,
            notes: customer.notes,
        }
    }
}
