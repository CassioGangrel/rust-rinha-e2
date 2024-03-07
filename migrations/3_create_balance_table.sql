create table if not exists balances (
	balance_id serial primary key,
	customer_id integer not null,
	name varchar(50) not null,
	value integer not null,
  	credit integer not null
);