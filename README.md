# Four.meme Sniper Bot

A Rust-based automated trading bot that snipes newly created tokens on the Four.meme platform on Binance Smart Chain (BSC). The bot can filter tokens by various criteria and automatically execute buy orders.

## Features

- **Real-time token detection**: Monitors blockchain events for new token creations via WebSocket
- **Flexible token filtering**: Filter tokens by creator, name, symbol, or other criteria
- **Automatic buying**: Purchases matching tokens with configurable BNB amount
- **One-shot execution**: Stops after successfully buying one token
- **Detailed logging**: Records all detected tokens and successful purchases

## Token Information Available

The bot extracts and can filter by the following token details from blockchain events:

- **Token Contract Address**: The smart contract address of the new token
- **Creator Address**: The wallet address that created the token
- **Token Name**: The full name of the token (e.g., "My Token Name")
- **Token Symbol/Ticker**: The trading symbol (e.g., "MTK")
- **Transaction Hash**: The blockchain transaction that created the token
- **Creation Amount**: Initial token supply

## Configuration

### Environment Variables (.env)
```bash
PRIVATE_KEY=your_wallet_private_key
RPC_URL=https://bsc-dataseed.binance.org
WSS_URL=wss://bsc-rpc.publicnode.com
TOKEN_MANAGER_ADDRESS=contract_address
HELPER3_ADDRESS=helper_contract_address
```

### Token Filtering Configuration (ticker.json)
Currently configured to buy all tokens, but can be modified to filter by:

```json
{
    "ticker": "FILTER_BY_SYMBOL",
    "buy_amount": "0.000001",
    "creator_filter": "0x123...", 
    "name_filter": "Token Name",
    "min_supply": "1000000"
}
```

### Available Filter Options

You can modify the bot to filter tokens based on:

1. **Token Symbol/Ticker**: Exact match with token trading symbol
2. **Creator Address**: Filter by specific wallet addresses that created tokens
3. **Token Name**: Filter by full token name (case-sensitive or case-insensitive)
4. **Transaction Hash**: Target specific creation transactions
5. **Minimum/Maximum Supply**: Filter by initial token supply amounts
6. **Creation Time**: Time-based filtering (within last X seconds/minutes)

## How It Works

1. **Listener**: Connects to BSC WebSocket and monitors `TokenCreate` events from Four.meme factory
2. **Detection**: Parses blockchain logs to extract token details (creator, name, symbol, address)
3. **Filtering**: Applies configured filters to determine if token should be purchased
4. **Trading**: Executes buy order using Four.meme's `buyTokenAMAP` function with BNB
5. **Completion**: Logs transaction details and exits after one successful purchase

## Usage

1. **Setup**: Configure `.env` file with your wallet and RPC details
2. **Configure**: Modify `ticker.json` to set buy amount and optional filters
3. **Run**: Execute the bot:
```bash
cargo run
```

4. **Monitor**: Watch console output and check `created_tokens.log` and `bought_tokens.log`

## Example Token Data Structure

When a new token is detected, the bot captures:
```rust
TokenEvent {
    contract: "0x4444e22132008714d484c6dfed4da3e09e25985a",  // Token address
    ticker: "DOGE",                                          // Symbol
    creator: Some("0x8a1e9c004321c2937c85062a4be12fb7fa2ae3de"), // Creator
    tx_hash: Some("0xe69809348a131dcbb0b701a2fdbdecbb6cb3857488666d5ccf51ea22b4ecdc86"),
    buy_amount: 0.000001                                     // BNB amount to spend
}
```

## Filtering Examples

To modify the bot for specific filtering, you can check the detection logic in `listener.rs`:

```rust
// Filter by exact symbol match
if token_event.ticker == "DOGE" { /* buy */ }

// Filter by creator address
if token_event.creator == Some(target_creator_address) { /* buy */ }

// Filter by name contains
if token_event.name.contains("PEPE") { /* buy */ }
```

## Requirements

- Rust 1.70+
- BNB balance (recommended 0.1+ BNB for gas fees and purchases)
- Valid wallet private key with BSC network access
- Stable internet connection for WebSocket streaming

## Architecture

- **`listener.rs`**: Blockchain event monitoring and token data extraction
- **`trader.rs`**: Smart contract interaction and buy execution via Four.meme
- **`main.rs`**: Application orchestration and task management
- **`types.rs`**: Data structures for token events and bot communication
- **`config.rs`**: Environment and configuration management

## Log Files

- **`created_tokens.log`**: All detected token creations with full details
- **`bought_tokens.log`**: Successfully purchased tokens and transaction hashes

## Notes

- Currently configured to buy the first detected token regardless of criteria
- Uses `buyTokenAMAP` for maximum token acquisition with fixed BNB amount
- Includes gas estimation and error handling for robust operation
- Bot stops automatically after one successful purchase to prevent over-buying

## Developer: [@Manokil](https://t.me/Rust0x_726)