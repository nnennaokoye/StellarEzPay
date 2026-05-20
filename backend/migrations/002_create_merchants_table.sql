-- Create merchants table
CREATE TABLE merchants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    business_name VARCHAR(255) NOT NULL,
    business_type VARCHAR(20) NOT NULL CHECK (business_type IN ('retail', 'restaurant', 'service', 'online', 'marketplace', 'other')),
    registration_number VARCHAR(100),
    tax_id VARCHAR(50),
    description TEXT,
    website_url VARCHAR(500),
    logo_url VARCHAR(500),
    business_address VARCHAR(500) NOT NULL,
    business_city VARCHAR(100) NOT NULL,
    business_state VARCHAR(100) NOT NULL,
    business_country VARCHAR(100) NOT NULL,
    business_postal_code VARCHAR(20) NOT NULL,
    contact_email VARCHAR(255) NOT NULL,
    contact_phone VARCHAR(20) NOT NULL,
    is_verified BOOLEAN DEFAULT FALSE NOT NULL,
    verification_status VARCHAR(20) DEFAULT 'pending' NOT NULL CHECK (verification_status IN ('pending', 'approved', 'rejected', 'under_review')),
    commission_rate DECIMAL(5, 4) DEFAULT 0.0000 NOT NULL,
    daily_transaction_limit BIGINT,
    monthly_transaction_limit BIGINT,
    deleted_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- Create indexes for frequently queried fields
CREATE INDEX idx_merchants_user_id ON merchants(user_id);
CREATE INDEX idx_merchants_business_name ON merchants(business_name);
CREATE INDEX idx_merchants_business_type ON merchants(business_type);
CREATE INDEX idx_merchants_is_verified ON merchants(is_verified);
CREATE INDEX idx_merchants_verification_status ON merchants(verification_status);
CREATE INDEX idx_merchants_deleted_at ON merchants(deleted_at);

-- Create trigger for updated_at
CREATE TRIGGER update_merchants_updated_at BEFORE UPDATE ON merchants
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
