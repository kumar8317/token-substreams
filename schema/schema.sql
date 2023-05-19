-- create table if not exists account 
-- (
--     id TEXT NOT NULL PRIMARY KEY
-- );

create table if not exists erc721_collection 
(
    id TEXT NOT NULL PRIMARY KEY,
    "name" TEXT,
    symbol TEXT ,
    supports_metadata Boolean,
    owner_address TEXT
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

create table if not exists erc1155_collection (
    id TEXT NOT NULL PRIMARY KEY,
    owner_address TEXT
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
     operator text
);

create table if not exists erc1155_balance
(   
    id TEXT NOT NULL PRIMARY KEY,
    "collection" TEXT NOT NULL,
    token TEXT NOT NULL,
    account TEXT,
    quantity TEXT
);

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

create table if not exists erc20_balance
(   
    id TEXT NOT NULL PRIMARY KEY,
    "contract" TEXT NOT NULL,
    account TEXT,
    quantity TEXT,
    block_number bigint
);

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
    transaction_type int
);

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

create table if not exists cursors
(
    id         TEXT not null constraint cursor_pk primary key,
    cursor     TEXT,
    block_num  BIGINT,
    block_id   TEXT
);
