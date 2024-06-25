# 99. Tendermint And CometBFT

<!-- 여기를 해야하나 말아야 하나 고민 중.. -->

**What are Tendermint and CometBFT?**

Tendermint modules **attend to consensus and networking**, which are important components of any blockchain. This frees developers to focus on the application level without descending into lower-level blockchain concerns such as peer discovery, block propagation, consensus, and transaction finalization

CometBFT is connected to the application by a socket protocol. ABCI provides a socket for applications written in other languages. If the application is written in the same language as the CometBFT implementation, the socket is not used.

<!-- -> 이래서, 같은 언어로 된거로 쓰는구나.. -->

코멧 더 알고싶으면 ㄱㄱ

For a deeper dive on consensus and CometBFT visit:

- This [**podcast on consensus systems (opens new window)**](https://softwareengineeringdaily.com/2018/03/26/consensus-systems-with-ethan-buchman/)with Ethan Buchman
- The [**CometBFT documentation on consensus**](https://docs.cometbft.com/v0.37/spec/consensus/)

-- 설명..
https://www.victorlamp.com/article/7387080850
Note that Tendermint only handles transaction bytes. It knows nothing about the meaning of these bytes. All Tendermint does is sequence these affairs in a certain way. Tendermint passes these bytes to the application through ABCI and looks forward to returning the code to inform it whether it has successfully processed the information contained in the transaction.

The following is the most important information in ABCI：

CheckTx: When Tendermint Core received a transaction, it was passed to the application to check whether some basic requirements were met. CheckTx is used to protect mempool at all nodes from spam transactions. . A special processing program called AnteHandler is used to perform a series of verification steps, such as checking whether there are sufficient fees and verifying the signature. If the inspection is valid, the transaction will be added to mempool and forwarded to the equivalent node. Please note that transactions are not processed in CheckTx （ that is, there is no modification of the state ） because they have not been included in a block.

DeliverTx: When Tendermint Core receives a valid block, each transaction in the block will be passed to the application through DeliverTx for processing. It was at this stage that a state conversion took place. AnteHandler performed again with RPC, the actual news service for each news in the business.

BeginBlock/EndBlock: The news is executed at the beginning and end of each block, regardless of whether the block contains a transaction. It is useful for automatic execution that triggers logic. But be cautious, because calculating the high-cost cycle may slow down the block chain, if the cycle is unlimited, it will even freeze it.

Find more detailed views of the ABCI method from the Tendermint document.

Any application built on Tendermint needs to implement the ABCI interface to communicate with the local Tendermint engine at the bottom. Fortunately, there is no need to realize the ABCI interface, and Cosmos SDK provides its template implementation in the form of baseapp.
