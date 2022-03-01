Pre-requisites for DataBase.
- Install postgresql with "local_user" and "local_user_password"
- Install diesel_cli: 
	cargo install diesel_cli --no-default-features --features postgres
- Into ./db-bank modify .env file:
	DATABASE_URL=postgres://"local_user":"local_user_password"@localhost/db_bank
     run:
	diesel setup
	diesel migration redo or diesel migration run 

Pre-requisites for API framework
- Into ./bank_ground modify Rocket.toml:
	db_bankbase = {url = "postgres://"local_user":"local_user_password"@localhost/db_bank"}
     run:	
	rustup override set nightly
	

