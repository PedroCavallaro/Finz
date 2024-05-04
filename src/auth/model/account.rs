use time::Date;
use uuid::Uuid;

pub struct Account {
    id: Uuid,
    balance: f64,
    currency_type: String,
    provider_id: String,
    created_at: Date,
}
