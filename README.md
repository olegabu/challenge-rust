# Hyperlane Rust Challenge

This repo hosts Rust and Solidity code for the soluton to [Abacus Works Rust Challenge](https://docs.google.com/document/d/1JGVZPSZ6vFzeFjIhOvxdXiU7-FzTQfFb5mq8UVrL4C4/edit#heading=h.a9izzxpgeg32).

## Directory Structure

The project is structured as a mixed Rust workspace with a Foundry project under
`contracts/` and typesafe auto-generated bindings to the contracts under
`bindings/`.

```
├── Cargo.toml
├── app // <-- Rust application logic
├── contracts // <- The smart contracts + tests using Foundry
├── bindings // <-- Generated bindings to the smart contracts' abis (like Typechain)
```

## Testing

Given the repository contains both Solidity and Rust code, there's 2 different
workflows.

### Solidity

Move to [contracts](./contracts) folder.

Forge is using submodules to manage dependencies. Initialize the dependencies.

```bash
forge install
```

Build contracts in [src](./src) and test with [Messaging.t.sol](test/Messaging.t.sol).

```bash
forge build &&
forge test
```

### Demo

Let's use tools from [Foundry](https://getfoundry.sh) suite: 
[forge](https://book.getfoundry.sh/forge/) to build and test, 
[cast](https://book.getfoundry.sh/cast/) to send transactions.
See the [book](https://book.getfoundry.sh/getting-started/installation.html) for instructions on how to install and use Foundry.


#### Initialize

I created *demo* user account on Sepolia and Mumbai networks to demo sending of a message. 
This user will deploy and call contracts, here are his account and private key, save them into env variables.

```bash
export demo=0xd24fC1ddb91f4C5179b3f2e1a64816eBDEEE4dC0 &&
export demo_key=c1836c120a271f4633073501c04cc93a6ee2ba3b267847cb0fc90e29765d1694
```

Let's also save rpc urls and constants into environment to be used in our commands.

```bash
export sepolia=https://eth-sepolia.g.alchemy.com/v2/demo &&
export mumbai=https://rpc-mumbai.maticvigil.com &&
export mumbai_id=`(cast chain-id --rpc-url $mumbai)` &&
export mailbox=0xCC737a94FecaeC165AbCf12dED095BB13F037685
```

#### Contracts

Deploy [HyperlaneMessageSender](./contracts/src/HyperlaneMessageSender.sol) to Sepolia
and [HyperlaneMessageReceiver](./contracts/src/HyperlaneMessageReceiver.sol) to Mumbai.
Both of the constructors take the address of Mailbox contract on their respective chain, 
it happens to be the same, see [Hyperlane's docs](https://docs.hyperlane.xyz/docs/resources/addresses).
Remember their addresses into `$sender` and `$receiver`; 
for that we get json output from *forge* and parse it with [jq](https://jqlang.github.io/jq/download/).

```bash
export sender=`(forge create --rpc-url $sepolia --private-key=$demo_key HyperlaneMessageSender --constructor-args $mailbox --json | jq -r .deployedTo)` &&
export receiver=`(forge create --rpc-url $mumbai --private-key=$demo_key HyperlaneMessageReceiver --constructor-args $mailbox --json | jq -r .deployedTo)`
```

Here's the sender in [Sepolia](https://sepolia.etherscan.io/address/0x03C43cDDcfb0DF2a4E670c8a8beeDcE2BAaeC144) deployed at block 4243635
and receiver in [Mumbai](https://mumbai.polygonscan.com/address/0x6482cda5df7605b52592a3d04af1f7e3004262fe) at block 39868015.

#### Message

Note the 20 byte address of the receiver contract needs to be prepended to become a byte32.
Let's save this into `$recipient`.

```bash
export recipient=`(echo $receiver | sed 's/0x/000000000000000000000000/g')`
```

Send a message from Sepolia's sender contract to Mumbai's receiver with *cast*.

```bash
cast send --rpc-url $sepolia --private-key=$demo_key $sender "sendString(uint32,bytes32,string)" $mumbai_id $recipient "hello from foundry"
```

See this message delivered on [Hyperlane Explorer](https://explorer.hyperlane.xyz/message/0xbdd6b25676fdbe6ab20a0895765bd877d246395b8f4ab370f5f7ccefa7a332e7).

#### Logs

Let's query Sepolia and Mumbai for the event logs of our sender and receiver contracts, starting from the blocks they were created at.
The sender has emitted 

```bash
cast logs --rpc-url $sepolia --from-block 4243635 --address $sender &&
cast logs --rpc-url $mumbai --from-block 39868015 --address $receiver
```

### Rust

```bash
cargo test
```

## Generating Rust bindings to the contracts

Rust bindings to the contracts can be generated via `forge bind`, which requires
first building your contracts. Move back to the project's root.

```bash
forge build --root ./contracts &&
forge bind --bindings-path ./bindings --root ./contracts --crate-name bindings --overwrite
```

Any follow-on calls to `forge bind` will check that the generated bindings match
the ones under the build files. If you want to re-generate your bindings, pass
the `--overwrite` flag to your `forge bind` command.

## Installing Foundry

First run the command below to get `foundryup`, the Foundry toolchain installer:

```sh
curl -L https://foundry.paradigm.xyz | bash
```

Then, in a new terminal session or after reloading your `PATH`, run it to get
the latest `forge` and `cast` binaries:

```sh
foundryup
```

For more, see the official
[docs](https://github.com/gakonst/foundry#installation).
