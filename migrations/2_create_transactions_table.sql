create table if not exists transactions (
	transaction_id uuid primary key default uuid_generate_v4(),
	customer_id integer not null,
	value integer not null,
	type char(1) not null,
	description varchar(10) not null,
	created_at timestamp not null default now()
);