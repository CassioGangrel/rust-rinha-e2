create or replace function update_balances()
returns trigger as
$$
begin
    if new.type = 'c' then
        update balances set value = value + new.value where customer_id = new.customer_id;
    elsif new.type = 'd' then
        if (select value - new.value from balances where customer_id = new.customer_id) < (select credit * -1 from balances where customer_id = new.customer_id) then
            raise exception 'saldo insuficiente para esta transação!';
        else
            update balances set value = value - new.value where customer_id = new.customer_id;
        end if;
    end if;
    return new;
end;
$$
language plpgsql;