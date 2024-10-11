# ic-alloy basic template

[Alloy](https://alloy.rs/) is the next generation of Ethereum support libraries,
written in Rust and designed for scalability and performance. Alloy is a rewrite
of of [ethers-rs](https://github.com/gakonst/ethers-rs) from the ground up.
Alloy includes built in support for transports like HTTP, WebSockets and IPC.

**Alloy now also works with the Internet Computer (ICP)!**

This canister implements three methods:

- `get_address`: Generates an Ethereum for an IC principal.
- `get_balance`: Returns the ETH balance of the Ethereum address controlled by a
  principal.
- `send_eth`: Sends ETH from the canister Ethereum address to any recipient.

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
