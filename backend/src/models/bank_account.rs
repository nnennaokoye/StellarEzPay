use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BankAccount {
    pub id: Uuid,
    pub user_id: Uuid,
    pub account_holder_name: String,
    pub bank_name: String,
    pub account_number: String,
    pub routing_number: String,
    pub account_type: BankAccountType,
    pub country: String,
    pub currency: String,
    pub is_primary: bool,
    pub is_verified: bool,
    pub verification_status: VerificationStatus,
    pub verification_document_id: Option<String>,
    pub daily_withdrawal_limit: Option<i64>,
    pub monthly_withdrawal_limit: Option<i64>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "bank_account_type", rename_all = "lowercase")]
pub enum BankAccountType {
    Checking,
    Savings,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "verification_status", rename_all = "lowercase")]
pub enum VerificationStatus {
    Pending,
    Verified,
    Failed,
}

impl BankAccount {
    pub fn new(
        user_id: Uuid,
        account_holder_name: String,
        bank_name: String,
        account_number: String,
        routing_number: String,
        account_type: BankAccountType,
        country: String,
        currency: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            account_holder_name,
            bank_name,
            account_number,
            routing_number,
            account_type,
            country,
            currency,
            is_primary: false,
            is_verified: false,
            verification_status: VerificationStatus::Pending,
            verification_document_id: None,
            daily_withdrawal_limit: None,
            monthly_withdrawal_limit: None,
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

    pub fn set_primary(&mut self) {
        self.is_primary = true;
        self.updated_at = Utc::now();
    }

    pub fn mark_verified(&mut self) {
        self.is_verified = true;
        self.verification_status = VerificationStatus::Verified;
        self.updated_at = Utc::now();
    }

    pub fn mark_verification_failed(&mut self) {
        self.is_verified = false;
        self.verification_status = VerificationStatus::Failed;
        self.updated_at = Utc::now();
    }
}
