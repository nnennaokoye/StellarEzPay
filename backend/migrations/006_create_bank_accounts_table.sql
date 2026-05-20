-- Create bank_accounts table
CREATE TABLE bank_accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    account_holder_name VARCHAR(255) NOT NULL,
    bank_name VARCHAR(255) NOT NULL,
    account_number VARCHAR(50) NOT NULL,
    routing_number VARCHAR(50) NOT NULL,
    account_type VARCHAR(20) NOT NULL CHECK (account_type IN ('checking', 'savings')),
    country VARCHAR(100) NOT NULL,
    currency VARCHAR(10) NOT NULL,
    is_primary BOOLEAN DEFAULT FALSE NOT NULL,
    is_verified BOOLEAN DEFAULT FALSE NOT NULL,
    verification_status VARCHAR(20) DEFAULT 'pending' NOT NULL CHECK (verification_status IN ('pending', 'verified', 'failed')),
    verification_document_id VARCHAR(255),
    daily_withdrawal_limit BIGINT,
    monthly_withdrawal_limit BIGINT,
    deleted_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- Create indexes for frequently queried fields
CREATE INDEX idx_bank_accounts_user_id ON bank_accounts(user_id);
CREATE INDEX idx_bank_accounts_account_number ON bank_accounts(account_number);
CREATE INDEX idx_bank_accounts_routing_number ON bank_accounts(routing_number);
CREATE INDEX idx_bank_accounts_is_primary ON bank_accounts(is_primary);
CREATE INDEX idx_bank_accounts_is_verified ON bank_accounts(is_verified);
CREATE INDEX idx_bank_accounts_verification_status ON bank_accounts(verification_status);
CREATE INDEX idx_bank_accounts_deleted_at ON bank_accounts(deleted_at);

-- Create trigger for updated_at
CREATE TRIGGER update_bank_accounts_updated_at BEFORE UPDATE ON bank_accounts
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
