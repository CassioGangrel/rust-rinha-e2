do $$
begin
	insert into balances (customer_id, name, credit, value)
	values
		(1, 'o barato sai caro', 1000 * 100, 0),
		(2, 'zan corp ltda', 800 * 100, 0),
		(3, 'les cruders', 10000 * 100, 0),
		(4, 'padaria joia de cocaia', 100000 * 100, 0),
		(5, 'kid mais', 5000 * 100, 0);
end;
$$;