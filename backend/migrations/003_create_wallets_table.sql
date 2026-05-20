-- Create wallets table
CREATE TABLE wallets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    wallet_address VARCHAR(255) UNIQUE NOT NULL,
    wallet_type VARCHAR(20) NOT NULL CHECK (wallet_type IN ('stellar', 'embedded', 'hardware', 'paper')),
    stellar_public_key VARCHAR(255) UNIQUE NOT NULL,
    stellar_secret_key_encrypted TEXT,
    is_default BOOLEAN DEFAULT FALSE NOT NULL,
    is_active BOOLEAN DEFAULT TRUE NOT NULL,
    balance BIGINT DEFAULT 0 NOT NULL,
    currency VARCHAR(10) NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- Create indexes for frequently queried fields
CREATE INDEX idx_wallets_user_id ON wallets(user_id);
CREATE INDEX idx_wallets_wallet_address ON wallets(wallet_address);
CREATE INDEX idx_wallets_stellar_public_key ON wallets(stellar_public_key);
CREATE INDEX idx_wallets_wallet_type ON wallets(wallet_type);
CREATE INDEX idx_wallets_is_default ON wallets(is_default);
CREATE INDEX idx_wallets_is_active ON wallets(is_active);
CREATE INDEX idx_wallets_deleted_at ON wallets(deleted_at);

-- Create trigger for updated_at
CREATE TRIGGER update_wallets_updated_at BEFORE UPDATE ON wallets
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
