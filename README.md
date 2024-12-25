# Convertible.Capital.Contract

## Smart Bond

**Smart Bond** is Solana [onchain program](https://solana.com/docs/core/programs) (referred to as _smart contract_ in other blockchains) that stores executable business logic.

## Features

- Written in the [Rust](https://doc.rust-lang.org/book/) programming language.
- [Anchor](https://solana.com/docs/programs/anchor) framework for Solana program development.
- VS Code extensions: [rust-bundle](vscode:extension/1YiB.rust-bundle), [rust-analyzer](vscode:extension/rust-lang.rust-analyzer) .
- [Cargo](https://doc.rust-lang.org/cargo/) is the package manager.
- [Mocha](https://mochajs.org/) framework for writing the tests and [Chai](https://www.chaijs.com/) for assertions.
- Solana Verify CLI is the primary tool used to [verify](https://solana.com/developers/guides/advanced/verified-builds) builds.

## Development environment setup

These guidelines below cover the steps to set up your local environment for Solana development.

- https://solana.com/docs/intro/installation
- https://www.anchor-lang.com/docs/installation

## Tools versioning

Installing the latest tools and dependencies are not always guarantee successful program build and run. The following versions were used for initial stable product build. Use them in case of troubleshooting.

- solana-cli 1.18.24 (src:6b04e881; feat:3241752014, client:Agave)
- rustc 1.81.0 (eeb90cda1 2024-09-04)
- anchor-cli 0.30.1

## Build the program

This command compiles your entire Anchor project, including any smart contracts within the `programs/` directory.

```
anchor clean
anchor build
```

Upon successful build, Anchor generates several important files within the `target/` directory, including the compiled program in a _.so_ file (shared object).

## Run local validator

Validator is a local emulator for the Solana blockchain for building and testing Solana programs.

```
solana config set --url localhost
solana-test-validator
```

Optionally open the second termital for log streaming.

```
solana logs --url localhost
```

See [more](https://solana.com/developers/guides/getstarted/solana-test-validator) information about Solana test validator.

## Deploy to Localnet

By default, _Anchor.toml_ config file of the current project specifies the localnet cluster. The `anchor deploy` command sends **Smart Bond** program to the single-node cluster on your workstation.

```
anchor deploy
```

Now you can use this RPC URL `127.0.0.1:8899` to test your client dApp connection.

## Deploy to Devnet

Devnet deployment does not require any local test-validator. Program is persisted on-chain. Use _cluster_ flag to override the default deployment behavior.

```
anchor deploy --provider.cluster devnet
```

Now the program has been be deployed to the devnet cluster. It makes the program public for external users.

---

## Testing

The `anchor test` command automatically does program deployment to the local cluster, starts the validator and invokes mocha tests according to _Anchor.toml_ config file.

```
anchor test
```

Non-compliant tests will be marked as failing (❌) and passing as (✔️).

### About test prerequisites

The `anchor test` command simplifies deploying and testing but hides the real complexity. Pay attention on the **test.validator.clone** section in _Anchor.toml_. It clones data account of the pyth program to your localnet automatically before actual mocha tests start. It prevents _AccountNotInitialized_ error during test execution runtime.

Dependent account (such as Pyth data account) could be stored from the mainnet to a separate file and reused later as a "test dependency".

```
solana account 7UVimffxr9ow1uXYxsr4LHAcV58mLzhmwaeKvJ1pjLiE --url https://api.mainnet-beta.solana.com --output json > tests/pyth-price-account.json
```

Local test validator can reuse this data by re-initializing account address at the start-up.

```
solana-test-validator --reset --account 7UVimffxr9ow1uXYxsr4LHAcV58mLzhmwaeKvJ1pjLiE tests/pyth-price-account.json
```

When local validator configured correctly and program is deployed, the tests should run the same way as for `anchor test` command.

```
anchor test --skip-local-validator --skip-deploy
```

## Backward compatibility

Additionally for program testing [Solana Playground](https://beta.solpg.io/) can be used. It provides browser-based UI access to the program instructions and accounts. But it requires downgraded `0.29.0` tools and IDL version.

In `Anchor.toml` set lower toolchain version.

```
[toolchain]
anchor_version = "0.29.0"
```

In `Cargo.toml` comment out idl-build feature section.

```
[features]
#idl-build = ["anchor-lang/idl-build", "anchor-spl/idl-build"]
```

After re-building program artifacts will be compatible with `anchor-cli-wasm 0.29.0`.

## Audits

Solana program audit is examination and evaluation of the functionalities within **Smart Bond** contract:

| Program    | Last Audit | Version          |
| ---------- | ---------- | ---------------- |
| Smart Bond | 2025-XX-XX | Vishnu (fffffff) |

## Acknowledgment

Show your appreciation to those who have contributed to the project: _Ricco_, _Air Crew_,
_Ice Beaver_, _Trinitron_, _Skalda_. You receive our group photo right after contribution.

[!["Buy Us A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://buymeacoffee.com/trinitron)

## License

SPDX-License-Identifier: [MIT](https://choosealicense.com/licenses/mit/).
