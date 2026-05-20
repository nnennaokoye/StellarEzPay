-- Create audit_logs table
CREATE TABLE audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    action VARCHAR(50) NOT NULL CHECK (action IN (
        'user_created', 'user_updated', 'user_deleted', 'user_logged_in', 'user_logged_out',
        'merchant_created', 'merchant_updated', 'merchant_deleted',
        'wallet_created', 'wallet_updated', 'wallet_deleted',
        'transaction_created', 'transaction_updated', 'transaction_completed', 'transaction_failed',
        'payment_request_created', 'payment_request_updated', 'payment_request_completed', 'payment_request_cancelled',
        'bank_account_created', 'bank_account_updated', 'bank_account_deleted', 'bank_account_verified',
        'kyc_submitted', 'kyc_approved', 'kyc_rejected',
        'password_changed', 'email_verified', 'password_reset_requested', 'password_reset_completed'
    )),
    entity_type VARCHAR(100) NOT NULL,
    entity_id UUID,
    ip_address VARCHAR(45),
    user_agent TEXT,
    details JSONB,
    status VARCHAR(20) NOT NULL CHECK (status IN ('success', 'failure')),
    error_message TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- Create indexes for frequently queried fields
CREATE INDEX idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX idx_audit_logs_action ON audit_logs(action);
CREATE INDEX idx_audit_logs_entity_type ON audit_logs(entity_type);
CREATE INDEX idx_audit_logs_entity_id ON audit_logs(entity_id);
CREATE INDEX idx_audit_logs_status ON audit_logs(status);
CREATE INDEX idx_audit_logs_created_at ON audit_logs(created_at);
CREATE INDEX idx_audit_logs_ip_address ON audit_logs(ip_address);
