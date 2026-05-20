-- Create transactions table
CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    from_wallet_id UUID NOT NULL REFERENCES wallets(id) ON DELETE RESTRICT,
    to_wallet_id UUID NOT NULL REFERENCES wallets(id) ON DELETE RESTRICT,
    payment_request_id UUID REFERENCES payment_requests(id) ON DELETE SET NULL,
    amount BIGINT NOT NULL,
    currency VARCHAR(10) NOT NULL,
    transaction_type VARCHAR(20) NOT NULL CHECK (transaction_type IN ('payment', 'deposit', 'withdrawal', 'transfer', 'refund')),
    status VARCHAR(20) DEFAULT 'pending' NOT NULL CHECK (status IN ('pending', 'processing', 'completed', 'failed', 'cancelled')),
    stellar_transaction_hash VARCHAR(255) UNIQUE,
    memo VARCHAR(255),
    fee BIGINT NOT NULL,
    description TEXT,
    metadata JSONB,
    completed_at TIMESTAMP WITH TIME ZONE,
    failed_at TIMESTAMP WITH TIME ZONE,
    failure_reason TEXT,
    deleted_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- Create indexes for frequently queried fields
CREATE INDEX idx_transactions_from_wallet_id ON transactions(from_wallet_id);
CREATE INDEX idx_transactions_to_wallet_id ON transactions(to_wallet_id);
CREATE INDEX idx_transactions_payment_request_id ON transactions(payment_request_id);
CREATE INDEX idx_transactions_status ON transactions(status);
CREATE INDEX idx_transactions_transaction_type ON transactions(transaction_type);
CREATE INDEX idx_transactions_stellar_transaction_hash ON transactions(stellar_transaction_hash);
CREATE INDEX idx_transactions_created_at ON transactions(created_at);
CREATE INDEX idx_transactions_deleted_at ON transactions(deleted_at);

-- Create trigger for updated_at
CREATE TRIGGER update_transactions_updated_at BEFORE UPDATE ON transactions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
