-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  password VARCHAR NOT NULL
);

CREATE TABLE accounts (
  id SERIAL PRIMARY KEY,
  num VARCHAR NOT NULL,
  amount INTEGER NOT NULL
);

CREATE TABLE transfers (
  account_id INTEGER REFERENCES accounts(id),
  user_id INTEGER REFERENCES users(id),
  selected INTEGER,
  PRIMARY KEY (account_id, user_id)
);

CREATE TABLE logtransfers (
  id SERIAL PRIMARY KEY,
  account_org VARCHAR NOT NULL,
  account_dest VARCHAR NOT NULL,
  amount INTEGER NOT NULL,
  date_tr DATE NOT NULL
);
