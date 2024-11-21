
## Prerequisites

- Rust and Cargo
- Docker (for optimization)
- Injective CLI
- Access to Injective testnet/mainnet

## Building

1. Build the contract:

```bash
cargo build --target wasm32-unknown-unknown --release
```

2. Optimize the contract:

```bash
docker run --rm -v "$(pwd)":/code \
--mount type=volume,source="$(basename "$(pwd)")cache",target=/code/target \
--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
cosmwasm/rust-optimizer:0.12.12
```

## Setting up Injective CLI

1. Install Injective CLI:

```bash
curl -L https://github.com/InjectiveLabs/injective-chain-releases/releases/download/v1.13.2-1723753267/linux-amd64.zip > cli.zip
unzip cli.zip
sudo mv injectived /usr/local/bin

//If you get a wasm error install CosmWasm
wget https://github.com/CosmWasm/wasmvm/releases/download/v1.3.0/libwasmvm.x86_64.so
sudo mv libwasmvm.x86_64.so /usr/lib/
```
2. Set up your wallet:

```bash
# Create new wallet
injectived keys add my-wallet
# Or import existing wallet
injectived keys add my-wallet --recover
```

3. Request testnet tokens:

https://testnet.faucet.injective.network/

## Deployment

1. Store the contract:

```bash
injectived tx wasm store artifacts/simple.wasm \
--from my-wallet \
--chain-id injective-888 \
--yes \
--gas-prices 500000000inj \
--gas 20000000 \
--node https://testnet.sentry.tm.injective.network:443
```

2. Get the code ID:

```bash
injectived query tx $TX_HASH \
--node https://testnet.sentry.tm.injective.network:443
```
Look for code ID in the response

```json
events:
- attributes:
  - index: true
    key: spender
    value: inj13r394d7unkm3mfwdet5v3a93p96slpuped9fzf
  - index: true
    key: amount
    value: 10000000000000000inj
  type: coin_spent
- attributes:
  - index: true
    key: receiver
    value: inj17xpfvakm2amg962yls6f84z3kell8c5l6s5ye9
  - index: true
    key: amount
    value: 10000000000000000inj
  type: coin_received
- attributes:
  - index: true
    key: recipient
    value: inj17xpfvakm2amg962yls6f84z3kell8c5l6s5ye9
  - index: true
    key: sender
    value: inj13r394d7unkm3mfwdet5v3a93p96slpuped9fzf
  - index: true
    key: amount
    value: 10000000000000000inj
  type: transfer
- attributes:
  - index: true
    key: sender
    value: inj13r394d7unkm3mfwdet5v3a93p96slpuped9fzf
  type: message
- attributes:
  - index: true
    key: fee
    value: 10000000000000000inj
  - index: true
    key: fee_payer
    value: inj13r394d7unkm3mfwdet5v3a93p96slpuped9fzf
  type: tx
- attributes:
  - index: true
    key: acc_seq
    value: inj13r394d7unkm3mfwdet5v3a93p96slpuped9fzf/0
  type: tx
- attributes:
  - index: true
    key: signature
    value: Ls+IjaZQxCbzwzFNrP9CiQlpzgAZtY5xaazyjz0xlZFx1epO2U/FsMsXOFgEUb99WF8ZZTKnjsbKwZxu40+t/gA=
  type: tx
- attributes:
  - index: true
    key: action
    value: /cosmwasm.wasm.v1.MsgStoreCode
  - index: true
    key: sender
    value: inj13r394d7unkm3mfwdet5v3a93p96slpuped9fzf
  - index: true
    key: module
    value: wasm
  - index: true
    key: msg_index
    value: "0"
  type: message
- attributes:
  - index: true
    key: access_config
    value: '{"permission":"Everybody","addresses":[]}'
  - index: true
    key: checksum
    value: '"zrDZUo+tj2Mx+os8JGXQh60YYiFNsCLNff+yFvB6pi0="'
  - index: true
+   key: code_id
+   value: '"14467"'
  - index: true
    key: creator
    value: '"inj13r394d7unkm3mfwdet5v3a93p96slpuped9fzf"'
  - index: true
    key: msg_index
    value: "0"
  type: cosmwasm.wasm.v1.EventCodeStored
gas_used: "1151252"
gas_wanted: "20000000"
height: "52595517"
info: ""
logs: []
raw_log: ""
timestamp: "2024-11-21T19:54:37Z"
```


3. Instantiate the contract:

```bash
injectived tx wasm instantiate $CODE_ID '{}' \
--from my-wallet \
--chain-id injective-888 \
--label "simple contract" \
--admin $(injectived keys show my-wallet -a) \
--gas-prices 500000000inj \
--gas 20000000 \
--yes \
--node https://testnet.sentry.tm.injective.network:443
```

## Interacting with the Contract

### Execute Messages

1. Call `plusTwo`:

```bash
injectived tx wasm execute $CONTRACT_ADDRESS '{"plus_two":{"a":"1","b":"2"}}' \
--from my-wallet \
--chain-id injective-888 \
--gas-prices 500000000inj \
--gas 20000000 \
--yes \
--node https://testnet.sentry.tm.injective.network:443
```

2. Call simple:

```bash
injectived tx wasm execute $CONTRACT_ADDRESS \
'{"simple":{"input":"123"}}' \
--from my-wallet \
--chain-id injective-888 \
--gas-prices 500000000inj \
--gas 20000000 \
--yes \
--node https://testnet.sentry.tm.injective.network:443
```

### Query Messages

1. Query `plusTwo`:

```bash
injectived query wasm contract-state smart $CONTRACT_ADDRESS \
'{"plus_two":{"a":"100","b":"200"}}' \
--node https://testnet.sentry.tm.injective.network:443
```

### ExecuteMsg

```json
// Simple
{
"simple": {
"input": "123"
}
}
// PlusTwo
{
"plus_two": {
"a": "100",
"b": "200"
    }
}
```

### QueryMsg

```json
{
"plus_two": {
"a": "100",
"b": "200"
}
}
```

## Schema Generation

Generate the contract's JSON Schema files:

```bash
cargo run --example schema
```

This will create the following schema files in the `schema` directory:
- `instantiate_msg.json`: Schema for contract instantiation message
- `execute_msg.json`: Schema for execution messages
- `query_msg.json`: Schema for query messages
- `plus_two_response.json`: Schema for the plusTwo query response
