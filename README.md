# Basic Substrate Blockchain

A UTXO (`U`nspent `T`ransaction `O`utput) chain implementation on Substrate.

## Installation

### 1. Install or update Rust
```bash
curl https://sh.rustup.rs -sSf | sh

rustup update nightly
rustup target add 
rustup update stable
```

## UI Demo

In this UI demo, you will interact with the UTXO blockchain via the [Polkadot UI](https://substrate.dev/docs/en/development/front-end/polkadot-js).

The following example takes you through a scenario where:
- Alice already owns a UTXO of value 100 upon genesis
- Alice sends Bob a UTXO with value 50, tipping the remainder to validators

1. Compile and build a release node
```bash
cargo +nightly build --release
```

2. Start a node. The `--dev` flag will start a single mining node, and the `--tmp` flag will start it in a new temporary directory.
```bash
./target/release/utxo-workshop --dev --tmp
```

3. In the console note the helper printouts. In particular, notice the default account `Alice` already has `100 UTXO` within the genesis block.

4. Open [Polkadot JS](https://polkadot.js.org/apps/#/settings) making sure the client is connected to your local node by going to Settings > General and selecting `Local Node` in the `remote node` dropdown.

5. **Declare custom datatypes in PolkadotJS** as the frontend cannot automatically detect this information. To do this, go to Settings > Developer tab and paste in the following JSON:

```json
{
  "Address": "AccountId",
  "LookupSource": "AccountId",
  "Value": "u128",
  "TransactionInput": {
    "outpoint": "Hash",
    "sigscript": "H512"
  },
  "TransactionOutput": {
    "value": "Value",
    "pubkey": "Hash"
  },
  "Transaction": {
    "inputs": "Vec<TransactionInput>",
    "outputs": "Vec<TransactionOutput>"
  },
  "Difficulty": "U256",
  "DifficultyAndTimestamp": {
    "difficulty": "Difficulty",
    "timestamp": "Moment"
  },
  "Public": "H256"
}
```

6. **Confirm that Alice already has 100 UTXO at genesis**. In `Chain State` > `Storage`, select `utxo`. Input the hash `0x76584168d10a20084082ed80ec71e2a783abbb8dd6eb9d4893b089228498e9ff`. Click the `+` notation to query blockchain state.

    Notice that:
    - This UTXO has a value of `100`
    - This UTXO belongs to Alice's pubkey. You use the [subkey](https://substrate.dev/docs/en/next/development/tools/subkey#well-known-keys) tool to confirm that the pubkey indeed belongs to Alice

7. **Spend Alice's UTXO, giving 50 to Bob.** In the `Extrinsics` tab, invoke the `spend` function from the `utxo` pallet, using Alice as the transaction sender. Use the following input parameters:

    - outpoint: `0x76584168d10a20084082ed80ec71e2a783abbb8dd6eb9d4893b089228498e9ff`
    - sigscript: `0x6ceab99702c60b111c12c2867679c5555c00dcd4d6ab40efa01e3a65083bfb6c6f5c1ed3356d7141ec61894153b8ba7fb413bf1e990ed99ff6dee5da1b24fd83`
    - value: `50`
    - pubkey: `0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48`

    Send as an `unsigned` transaction. With UTXO blockchains, the proof is already in the `sigscript` input.

8. **Verify that your transaction succeeded**. In `Chain State`, look up the newly created UTXO hash: `0xdbc75ab8ee9b83dcbcea4695f9c42754d94e92c3c397d63b1bc627c2a2ef94e6` to verify that a new UTXO of 50, belonging to Bob, now exists! Also you can verify that Alice's original UTXO has been spent and no longer exists in UtxoStore.

