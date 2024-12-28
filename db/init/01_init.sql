-- Create accounts table
CREATE TABLE accounts (
    id SERIAL PRIMARY KEY,
    code VARCHAR(10) UNIQUE NOT NULL,
    name VARCHAR(100) NOT NULL,
    account_type VARCHAR(20) NOT NULL
);

-- Create journals table
CREATE TABLE journals (
    id SERIAL PRIMARY KEY,
    journal_number VARCHAR(20) UNIQUE NOT NULL,
    date TIMESTAMP NOT NULL,
    description TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create journal_entries table
CREATE TABLE journal_entries (
    id SERIAL PRIMARY KEY,
    journal_id INTEGER NOT NULL REFERENCES journals(id) ON DELETE CASCADE,
    account_id INTEGER NOT NULL REFERENCES accounts(id),
    is_debit BOOLEAN NOT NULL,
    amount INTEGER NOT NULL
);

-- Insert initial account data
INSERT INTO accounts (id, code, name, account_type) VALUES
    -- 資産の部
    (1, '1001', '現金', 'ASSET'),
    (2, '1002', '普通預金', 'ASSET'),
    (3, '1101', '備品', 'ASSET'),
    (4, '1102', 'パソコン', 'ASSET'),
    (5, '1103', 'ソフトウェア', 'ASSET'),
    -- 負債の部
    (6, '2001', '買掛金', 'LIABILITY'),
    (7, '2002', '未払金', 'LIABILITY'),
    (8, '2003', '預り金', 'LIABILITY'),
    (9, '2004', '所得税預り金', 'LIABILITY'),
    -- 純資産の部
    (10, '3001', '元入金', 'EQUITY'),
    (11, '3002', '事業主貸', 'EQUITY'),
    (12, '3003', '事業主借', 'EQUITY'),
    -- 収益の部
    (13, '4001', '売上', 'REVENUE'),
    (14, '4002', '受取利息', 'REVENUE'),
    -- 費用の部
    (15, '5001', '仕入', 'EXPENSE'),
    (16, '5002', '消耗品費', 'EXPENSE'),
    (17, '5003', '通信費', 'EXPENSE'),
    (18, '5004', '水道光熱費', 'EXPENSE'),
    (19, '5005', '地代家賃', 'EXPENSE'),
    (20, '5006', '接待交際費', 'EXPENSE'),
    (21, '5007', '損害保険料', 'EXPENSE'),
    (22, '5008', '減価償却費', 'EXPENSE'),
    (23, '5009', 'クラウド利用料', 'EXPENSE'),
    (24, '5010', '支払手数料', 'EXPENSE');

-- Reset the sequence for accounts id
SELECT setval('accounts_id_seq', (SELECT MAX(id) FROM accounts));