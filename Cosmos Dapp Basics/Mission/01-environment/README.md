# Cosmos-SDK dApp Development Environment

Set up a development environment for carrying out missions.

## Terminal

Warp, iTerm2, Terminal 등

In the example, the input is $ and the output is >. # represents an annotation.

```bash
$ echo A
> A
# Comment
```

## IDE

IDE can be used freely, but the mission example is based on VSCode.

VSCode https://code.visualstudio.com/

## Install nodejs and related packages

The dApp development mission uses 'nodejs', 'nextjs', and 'shadcn/ui' for front-end development, and the package manager uses 'yarn'.

```bash
#Install nodejs
https://nodejs.org/en

$ npm i -g yarn
```

## Wallet Installation

In Cosmos Ecosystem, there are [Cosmostation](https://chromewebstore.google.com/detail/cosmostation-wallet/fpkhgmpbidmiogeglndfbkegfdlnajnf), [Keplr](https://chromewebstore.google.com/detail/keplr/dmkamcknogkgcdfhhbddcghachkejeap), and other wallets.

In the mission, we will make good use of all wallets, but we will use Cosmostation wallets that support EVM transaction(Mission#9-EVM) to conduct examples.

## Test App Chain Information

For missions, Cosmos, Neutron, and Osmosis test nets are used. Prepare in advance through each chain information and Faucet information.

- Cosmoshub Testnet
  - Explorer
    - https://www.mintscan.io/cosmoshub-testnet
  - Faucet
    - https://testnet.ping.pub/cosmos/faucet
- Neutron Testnet
  - Explorer
    - https://www.mintscan.io/neutron-testnet
    - https://neutron.celat.one/pion-1
  - Faucet
    - https://docs.neutron.org/neutron/faq/#where-is-the-testnet-faucet
- Osmosis Testnet
  - Explorer
    - https://www.mintscan.io/osmosis-testnet
  - Faucet
    - https://testnet.ping.pub/osmosis/faucet
