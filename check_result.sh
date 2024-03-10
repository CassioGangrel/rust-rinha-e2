#! /bin/bash
curl http://localhost:9999/clientes/1/extrato | jq ".saldo" &
curl http://localhost:9999/clientes/2/extrato | jq ".saldo" &
curl http://localhost:9999/clientes/3/extrato | jq ".saldo" &
curl http://localhost:9999/clientes/4/extrato | jq ".saldo" &
curl http://localhost:9999/clientes/5/extrato | jq ".saldo" &
