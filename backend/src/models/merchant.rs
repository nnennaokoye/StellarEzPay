use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Merchant {
    pub id: Uuid,
    pub user_id: Uuid,
    pub business_name: String,
    pub business_type: BusinessType,
    pub registration_number: Option<String>,
    pub tax_id: Option<String>,
    pub description: Option<String>,
    pub website_url: Option<String>,
    pub logo_url: Option<String>,
    pub business_address: String,
    pub business_city: String,
    pub business_state: String,
    pub business_country: String,
    pub business_postal_code: String,
    pub contact_email: String,
    pub contact_phone: String,
    pub is_verified: bool,
    pub verification_status: VerificationStatus,
    pub commission_rate: f64,
    pub daily_transaction_limit: Option<i64>,
    pub monthly_transaction_limit: Option<i64>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "business_type", rename_all = "lowercase")]
pub enum BusinessType {
    Retail,
    Restaurant,
    Service,
    Online,
    Marketplace,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "verification_status", rename_all = "lowercase")]
pub enum VerificationStatus {
    Pending,
    Approved,
    Rejected,
    UnderReview,
}

impl Merchant {
    pub fn new(
        user_id: Uuid,
        business_name: String,
        business_type: BusinessType,
        business_address: String,
        business_city: String,
        business_state: String,
        business_country: String,
        business_postal_code: String,
        contact_email: String,
        contact_phone: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            business_name,
            business_type,
            registration_number: None,
            tax_id: None,
            description: None,
            website_url: None,
            logo_url: None,
            business_address,
            business_city,
            business_state,
            business_country,
            business_postal_code,
            contact_email,
            contact_phone,
            is_verified: false,
            verification_status: VerificationStatus::Pending,
            commission_rate: 0.0,
            daily_transaction_limit: None,
            monthly_transaction_limit: None,
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
}
