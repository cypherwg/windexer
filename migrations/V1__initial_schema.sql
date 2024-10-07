CREATE KEYSPACE IF NOT EXISTS windexer WITH replication = {'class': 'SimpleStrategy', 'replication_factor': 1};

USE windexer;

CREATE TABLE IF NOT EXISTS accounts (
    pubkey text PRIMARY KEY,
    lamports bigint,
    owner text,
    executable boolean,
    rent_epoch bigint,
    data blob
);

CREATE TABLE IF NOT EXISTS transactions (
    signature text PRIMARY KEY,
    slot bigint,
    err text,
    memo text,
    block_time timestamp
);