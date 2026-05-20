use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AuditLog {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub action: AuditAction,
    pub entity_type: String,
    pub entity_id: Option<Uuid>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub details: Option<serde_json::Value>,
    pub status: AuditStatus,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "audit_action", rename_all = "lowercase")]
pub enum AuditAction {
    UserCreated,
    UserUpdated,
    UserDeleted,
    UserLoggedIn,
    UserLoggedOut,
    MerchantCreated,
    MerchantUpdated,
    MerchantDeleted,
    WalletCreated,
    WalletUpdated,
    WalletDeleted,
    TransactionCreated,
    TransactionUpdated,
    TransactionCompleted,
    TransactionFailed,
    PaymentRequestCreated,
    PaymentRequestUpdated,
    PaymentRequestCompleted,
    PaymentRequestCancelled,
    BankAccountCreated,
    BankAccountUpdated,
    BankAccountDeleted,
    BankAccountVerified,
    KYCSubmitted,
    KYCApproved,
    KYCRejected,
    PasswordChanged,
    EmailVerified,
    PasswordResetRequested,
    PasswordResetCompleted,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "audit_status", rename_all = "lowercase")]
pub enum AuditStatus {
    Success,
    Failure,
}

impl AuditLog {
    pub fn new(
        user_id: Option<Uuid>,
        action: AuditAction,
        entity_type: String,
        entity_id: Option<Uuid>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            action,
            entity_type,
            entity_id,
            ip_address: None,
            user_agent: None,
            details: None,
            status: AuditStatus::Success,
            error_message: None,
            created_at: Utc::now(),
        }
    }

    pub fn with_ip_address(mut self, ip_address: String) -> Self {
        self.ip_address = Some(ip_address);
        self
    }

    pub fn with_user_agent(mut self, user_agent: String) -> Self {
        self.user_agent = Some(user_agent);
        self
    }

    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }

    pub fn mark_failure(mut self, error_message: String) -> Self {
        self.status = AuditStatus::Failure;
        self.error_message = Some(error_message);
        self
    }
}
