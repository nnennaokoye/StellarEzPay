use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Wallet {
    pub id: Uuid,
    pub user_id: Uuid,
    pub wallet_address: String,
    pub wallet_type: WalletType,
    pub stellar_public_key: String,
    pub stellar_secret_key_encrypted: Option<String>,
    pub is_default: bool,
    pub is_active: bool,
    pub balance: i64,
    pub currency: String,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "wallet_type", rename_all = "lowercase")]
pub enum WalletType {
    Stellar,
    Embedded,
    Hardware,
    Paper,
}

impl Wallet {
    pub fn new(
        user_id: Uuid,
        wallet_address: String,
        wallet_type: WalletType,
        stellar_public_key: String,
        currency: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            wallet_address,
            wallet_type,
            stellar_public_key,
            stellar_secret_key_encrypted: None,
            is_default: false,
            is_active: true,
            balance: 0,
            currency,
            deleted_at: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn soft_delete(&mut self) {
        self.deleted_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    pub fn restore(&mut self) {
        self.deleted_at = None;
        self.updated_at = Utc::now();
    }

    pub fn set_default(&mut self) {
        self.is_default = true;
        self.updated_at = Utc::now();
    }

    pub fn update_balance(&mut self, new_balance: i64) {
        self.balance = new_balance;
        self.updated_at = Utc::now();
    }
}
