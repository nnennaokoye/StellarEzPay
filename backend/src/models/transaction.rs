use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub id: Uuid,
    pub from_wallet_id: Uuid,
    pub to_wallet_id: Uuid,
    pub payment_request_id: Option<Uuid>,
    pub amount: i64,
    pub currency: String,
    pub transaction_type: TransactionType,
    pub status: TransactionStatus,
    pub stellar_transaction_hash: Option<String>,
    pub memo: Option<String>,
    pub fee: i64,
    pub description: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub completed_at: Option<DateTime<Utc>>,
    pub failed_at: Option<DateTime<Utc>>,
    pub failure_reason: Option<String>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "transaction_type", rename_all = "lowercase")]
pub enum TransactionType {
    Payment,
    Deposit,
    Withdrawal,
    Transfer,
    Refund,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "transaction_status", rename_all = "lowercase")]
pub enum TransactionStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

impl Transaction {
    pub fn new(
        from_wallet_id: Uuid,
        to_wallet_id: Uuid,
        amount: i64,
        currency: String,
        transaction_type: TransactionType,
        fee: i64,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            from_wallet_id,
            to_wallet_id,
            payment_request_id: None,
            amount,
            currency,
            transaction_type,
            status: TransactionStatus::Pending,
            stellar_transaction_hash: None,
            memo: None,
            fee,
            description: None,
            metadata: None,
            completed_at: None,
            failed_at: None,
            failure_reason: None,
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

    pub fn mark_completed(&mut self, stellar_hash: String) {
        self.status = TransactionStatus::Completed;
        self.stellar_transaction_hash = Some(stellar_hash);
        self.completed_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    pub fn mark_failed(&mut self, reason: String) {
        self.status = TransactionStatus::Failed;
        self.failed_at = Some(Utc::now());
        self.failure_reason = Some(reason);
        self.updated_at = Utc::now();
    }
}
