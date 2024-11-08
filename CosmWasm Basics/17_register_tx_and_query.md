# Execute register transactions
## Table of Contents
0. Execute register transactions
1. Inquire `ResolveRecord` query
2. Submit register transaction execution results

## 0. Execute register transactions
Now it's time to use the nameservice properly. Select the `Execute` tab, select `register` and type name to write a message:
```json
{
  "register": {
    "name": "hello"
  }
}
```

And the important thing is that we should send the minimum amount that we previously set as the initialization message. 100untrn is equal to 0.0001ntrn, so you have to input it into attach funds and send it. The overall content is as follows:
![](./assets/33a_contract_register_execute.png)

After this is executed, a [successful transaction](https://neutron.celat.one/pion-1/txs/B0D62692B1ACCBAD92DD040DC3F8B3746BF53A398A567AA67854351BEE6A65) is issued. Still, let's look up the `ResolveRecord` query to make sure the data is stored properly.

## 1. Query `Resolve Record`
Enter the name to match the query message format of `ResolveRecord` as follows:
```json
{
  "resolve_record": {
    "name": "hello"
  }
}
```

Then, the qurey result shows that the registration is usccessful:
```json
{
  "data": {
    "address": "neutron..."
  }
}
```

![](./assets/33a_contract_resolverecord_query.png)


If the unregistered name is serached, it retunrs null:
```json
{
  "data": {
    "address": null
  }
}
```

## 2. Submit register transaction execution results
You must submit the results of directly executing the register transaction as follows:
- transaction hash: [C1AED2C9966260AF037342D9321711BBDC302A154C41AECB7A14B14C032E4CA6](https://neutron.celat.one/pion-1/txs/C1AED2C9966260AF037342D9321711BBDC302A154C41AECB7A14B14C032E4CA6)