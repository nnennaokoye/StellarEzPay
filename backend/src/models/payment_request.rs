use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PaymentRequest {
    pub id: Uuid,
    pub merchant_id: Uuid,
    pub merchant_wallet_id: Uuid,
    pub amount: i64,
    pub currency: String,
    pub description: String,
    pub reference: Option<String>,
    pub status: PaymentRequestStatus,
    pub expires_at: Option<DateTime<Utc>>,
    pub customer_email: Option<String>,
    pub customer_phone: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub completed_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub cancellation_reason: Option<String>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "payment_request_status", rename_all = "lowercase")]
pub enum PaymentRequestStatus {
    Pending,
    Completed,
    Cancelled,
    Expired,
}

impl PaymentRequest {
    pub fn new(
        merchant_id: Uuid,
        merchant_wallet_id: Uuid,
        amount: i64,
        currency: String,
        description: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            merchant_id,
            merchant_wallet_id,
            amount,
            currency,
            description,
            reference: None,
            status: PaymentRequestStatus::Pending,
            expires_at: None,
            customer_email: None,
            customer_phone: None,
            metadata: None,
            completed_at: None,
            cancelled_at: None,
            cancellation_reason: None,
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

    pub fn mark_completed(&mut self) {
        self.status = PaymentRequestStatus::Completed;
        self.completed_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    pub fn mark_cancelled(&mut self, reason: String) {
        self.status = PaymentRequestStatus::Cancelled;
        self.cancelled_at = Some(Utc::now());
        self.cancellation_reason = Some(reason);
        self.updated_at = Utc::now();
    }

    pub fn mark_expired(&mut self) {
        self.status = PaymentRequestStatus::Expired;
        self.updated_at = Utc::now();
    }
}
