-- create table if not exists account 
-- (
--     id TEXT NOT NULL PRIMARY KEY
-- );

create table if not exists erc721_collection 
(
    id TEXT NOT NULL PRIMARY KEY,
    "name" TEXT,
    symbol TEXT ,
    supports_metadata Boolean
);

create table if not exists erc721_token
(   
    id TEXT NOT NULL PRIMARY KEY,
    "collection" TEXT NOT NULL,
    token_id TEXT NOT NULL,
    metadata_uri TEXT ,
    minter_address TEXT ,
    owner_address TEXT,
    block_number_minted bigint ,
    block_number bigint,
    mint_trx TEXT
);
CREATE INDEX IF NOT EXISTS idx_erc721_token_collection ON erc721_token ("collection");
CREATE INDEX IF NOT EXISTS idx_erc721_token_owner_address ON erc721_token (owner_address);

create table if not exists erc721_transfer
(   
    id TEXT NOT NULL PRIMARY KEY,
    "collection" TEXT NOT NULL,
    token TEXT NOT NULL,
    trx_hash TEXT,
    from_address TEXT,
    to_address TEXT,
    log_index bigint,
    block_number bigint,
    block_hash TEXT,
    "timestamp" bigint,
    transaction_index int,
    transaction_type int,
    "value" text

);
CREATE INDEX IF NOT EXISTS idx_erc721_transfer_block_number ON erc721_transfer (block_number);
CREATE INDEX IF NOT EXISTS idx_erc721_transfer_timestamp ON erc721_transfer ("timestamp");
CREATE INDEX IF NOT EXISTS idx_erc721_transfer_collection ON erc721_transfer ("collection");
CREATE INDEX IF NOT EXISTS idx_erc721_transfer_from_address ON erc721_transfer ("from_address");
CREATE INDEX IF NOT EXISTS idx_erc721_transfer_to_address ON erc721_transfer ("to_address");
CREATE INDEX IF NOT EXISTS idx_erc721_transfer_token ON erc721_transfer (token);

create table if not exists erc721_operator
(   
    id TEXT NOT NULL PRIMARY KEY,
    "collection" TEXT NOT NULL,
    "owner" TEXT NOT NULL,
    operator TEXT,
    approved BOOLEAN
);

create table if not exists erc721_approval
(   
    id TEXT NOT NULL PRIMARY KEY,
    "token" TEXT NOT NULL,
    "owner" TEXT NOT NULL,
    approval TEXT
);

create table if not exists erc1155_token
(   
    id TEXT NOT NULL PRIMARY KEY,
    "collection" TEXT NOT NULL,
    token_id TEXT NOT NULL,
    metadata_uri TEXT ,
    minter_address TEXT ,
    owner_address TEXT,
    block_number_minted bigint ,
    block_number bigint,
    mint_trx TEXT
);
CREATE INDEX IF NOT EXISTS idx_erc1155_token_collection ON erc1155_token ("collection");
CREATE INDEX IF NOT EXISTS idx_erc1155_token_owner_address ON erc1155_token (owner_address);

create table if not exists erc1155_transfer
(   
    id TEXT NOT NULL PRIMARY KEY,
    "collection" TEXT NOT NULL,
    token TEXT NOT NULL,
    trx_hash TEXT,
    from_address TEXT,
    to_address TEXT,
    quantity TEXT,
    log_index bigint,
    block_number bigint,
    block_hash TEXT,
    "timestamp" bigint,
    transaction_index int,
    transaction_type int,
     "value" text,
     operator text,
     from_balance text,
     to_balance text
);
CREATE INDEX IF NOT EXISTS idx_erc1155_transfer_block_number ON erc1155_transfer (block_number);
CREATE INDEX IF NOT EXISTS idx_erc1155_transfer_timestamp ON erc1155_transfer ("timestamp");
CREATE INDEX IF NOT EXISTS idx_erc1155_transfer_collection ON erc1155_transfer ("collection");
CREATE INDEX IF NOT EXISTS idx_erc1155_transfer_from_address ON erc1155_transfer ("from_address");
CREATE INDEX IF NOT EXISTS idx_erc1155_transfer_to_address ON erc1155_transfer ("to_address");
CREATE INDEX IF NOT EXISTS idx_erc1155_transfer_token ON erc1155_transfer (token);
CREATE INDEX IF NOT EXISTS idx_erc1155_transfer_from_balance ON erc1155_transfer (from_balance);
CREATE INDEX IF NOT EXISTS idx_erc1155_transfer_to_balance ON erc1155_transfer (to_balance);

-- create table if not exists erc1155_balance
-- (   
--     id TEXT NOT NULL PRIMARY KEY,
--     "collection" TEXT NOT NULL,
--     token TEXT NOT NULL,
--     account TEXT,
--     quantity TEXT
-- );

create table if not exists erc1155_operator
(   
    id TEXT NOT NULL PRIMARY KEY,
    "collection" TEXT NOT NULL,
    "owner" TEXT NOT NULL,
    operator TEXT,
    approved BOOLEAN
);

create table if not exists erc20_contract (
    id TEXT NOT NULL PRIMARY KEY,
    "name" TEXT,
    symbol TEXT ,
    decimals bigint
    -- owner_address TEXT
);

-- create table if not exists erc20_balance
-- (   
--     id TEXT NOT NULL PRIMARY KEY,
--     "contract" TEXT NOT NULL,
--     account TEXT,
--     quantity TEXT,
--     block_number bigint
-- );

create table if not exists erc20_transfer
(   
    id TEXT NOT NULL PRIMARY KEY,
    "contract" TEXT NOT NULL,
    trx_hash TEXT,
    from_address TEXT,
    to_address TEXT,
    quantity TEXT,
    log_index bigint,
    block_number bigint,
    block_hash TEXT,
    "timestamp" bigint,
    transaction_index int,
    transaction_type int,
    from_balance TEXT,
    to_balance TEXT
);
CREATE INDEX IF NOT EXISTS idx_erc20_transfer_contract ON erc20_transfer ("contract");
CREATE INDEX IF NOT EXISTS idx_erc20_transfer_from_address ON erc20_transfer (from_address);
CREATE INDEX IF NOT EXISTS idx_erc20_transfer_to_address ON erc20_transfer (to_address);
CREATE INDEX IF NOT EXISTS idx_erc20_transfer_from_balance ON erc20_transfer (from_balance);
CREATE INDEX IF NOT EXISTS idx_erc20_transfer_to_balance ON erc20_transfer (to_balance);
CREATE INDEX IF NOT EXISTS idx_erc20_transfer_block_number ON erc20_transfer (block_number);
CREATE INDEX IF NOT EXISTS idx_erc20_transfer_timestamp ON erc20_transfer ("timestamp");

create table if not exists erc20_approval
(   
    id TEXT NOT NULL PRIMARY KEY,
    "contract" TEXT NOT NULL,
    "owner" TEXT,
    spender TEXT,
    quantity TEXT,
    trx_hash TEXT,
    block_number bigint,
    block_hash TEXT,
    block_timestamp bigint,
    log_index bigint,
    transaction_index int
);

CREATE INDEX IF NOT EXISTS idx_erc20_approval_contract ON erc20_approval ("contract");
CREATE INDEX IF NOT EXISTS idx_erc20_approval_owner ON erc20_approval ("owner");
CREATE INDEX IF NOT EXISTS idx_erc20_approval_spender ON erc20_approval (spender);
CREATE INDEX IF NOT EXISTS idx_erc20_approval_block_number ON erc20_approval (block_number);
CREATE INDEX IF NOT EXISTS idx_erc20_approval_block_timestamp ON erc20_approval (block_timestamp);

create table if not exists collection_owner
(   
    id TEXT NOT NULL PRIMARY KEY,
    owner_address TEXT,
    deploy_trx TEXT
);
CREATE INDEX IF NOT EXISTS idx_collection_owner_owner_address ON collection_owner (owner_address);

create table if not exists cursors
(
    id         TEXT not null constraint cursor_pk primary key,
    cursor     TEXT,
    block_num  BIGINT,
    block_id   TEXT
);
