# ic-alloy, basic Ethereum multi-user wallet

[Alloy](https://alloy.rs/) is the next generation of Ethereum support libraries,
written in Rust and designed for scalability and performance. Alloy is a rewrite
of of [ethers-rs](https://github.com/gakonst/ethers-rs) from the ground up.
Alloy includes built in support for transports like HTTP, WebSockets and IPC.

**Alloy now also works with the Internet Computer (ICP)!**

## Canister methods

### `get_address`

Get the Ethereum address for the calling principal or for the principal
specified in the call parameters.

Call signature:

```
get_address : (owner: opt principal) -> (text);
```

Get the Ethereum address for the calling principal:

```bash
dfx canister call basic_eth get_address
```

Get the Ethereum address for a specified principal:

```bash
dfx canister call basic_eth get_address '(opt principal "hkroy-sm7vs-yyjs7-ekppe-qqnwx-hm4zf-n7ybs-titsi-k6e3k-ucuiu-uqe")'
```

### `get_balance`

Returns the ETH balance of the Ethereum address controlled by a principal.

Call signature:

```
get_balance : (owner: opt principal) -> (text);
```

Get the ETH balance for the calling principal:

```bash
dfx canister call basic_eth get_balance
```

Get the ETH balance for a specified principal:

```bash
dfx canister call basic_eth get_balance '(opt principal "hkroy-sm7vs-yyjs7-ekppe-qqnwx-hm4zf-n7ybs-titsi-k6e3k-ucuiu-uqe")'
```

### `send_eth`

Sends ETH from the Ethereum controlled by the calling principal to any
recipient.

Call signature:

```
send_eth : (to: text, amount: Wei) -> (text);
```

Send ETH by specifying receiver address and ETH amount (in wei):

```bash
dfx canister call basic_eth send_eth '("0xa32aECda752cF4EF89956e83d60C04835d4FA867", 1)'

```

## Run locally

```bash
dfx start --background
dfx deploy
```

## Author

- [kristofer@kristoferlund.se](mailto:kristofer@kristoferlund.se)
- Twitter: [@kristoferlund](https://twitter.com/kristoferlund)
- Discord: kristoferkristofer
- Telegram: [@kristoferkristofer](https://t.me/kristoferkristofer)

## License

This project is licensed under the MIT License. See the LICENSE file for more
details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request if you
have any suggestions or improvements.
