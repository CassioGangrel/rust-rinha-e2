create trigger transactions_trigger
after insert
on transactions
for each row
execute function update_balances();