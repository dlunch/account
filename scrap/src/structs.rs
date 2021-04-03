use chrono::{offset::Utc, DateTime};

pub struct CardTransaction {
    pub transaction_id: String,
    pub date: DateTime<Utc>,
    pub amount: String,
    pub currency: String,
    pub merchant_id: Option<String>,
    pub merchant: String,
    pub month: i8,
    pub canceled: bool,
}

#[derive(Eq, PartialEq, Hash)]
pub struct Card {
    pub display_name: String,
    pub last4: String,
}
