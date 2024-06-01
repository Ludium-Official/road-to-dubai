# 99. Tendermint And CometBFT

여기를 해야하나 말아야 하나 고민 중..

**What are Tendermint and CometBFT?**

Tendermint modules **attend to consensus and networking**, which are important components of any blockchain. This frees developers to focus on the application level without descending into lower-level blockchain concerns such as peer discovery, block propagation, consensus, and transaction finalization

CometBFT is connected to the application by a socket protocol. ABCI provides a socket for applications written in other languages. If the application is written in the same language as the CometBFT implementation, the socket is not used.
-> 이래서, 같은 언어로 된거로 쓰는구나..

코멧 더 알고싶으면 ㄱㄱ

For a deeper dive on consensus and CometBFT visit:

- This [**podcast on consensus systems (opens new window)**](https://softwareengineeringdaily.com/2018/03/26/consensus-systems-with-ethan-buchman/)with Ethan Buchman
- The [**CometBFT documentation on consensus**](https://docs.cometbft.com/v0.37/spec/consensus/)
