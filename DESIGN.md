# Implementation details

Solana programs are "stateless", meaning that program accounts only contain the program's executable byte code. To store and modify additional data, new accounts must be created. These accounts are commonly referred to as _data accounts_.

## Bond account space reference

Bond data account structure (Vishnu version) contains 464 bytes of information. Plus 8 bytes for internal discriminator. Estimated minimal rent-exempt: ~0.004 SOL.
| Name | Type | Bytes | Description |
| -------------- | ----------- | ----- | -------------------------------------------- |
| id | u64 | 8 | Bond identifier. |
| bump | u8 | 1 | System security field. |
| issuer | Pubkey | 32 | Wallet address of bond issuer. |
| owner | Pubkey | 32 | Wallet address of bond owner (buyer). |
| vault | Pubkey | 32 | Vault address for holding collateral. |
| name | String | 36 | Name of the bond. |
| amount_b | u64 | 8 | Face value (loan amount). |
| mint_b | Pubkey | 32 | Face mint (currency of the loan). |
| amount_a | u64 | 8 | Collateral amount. |
| mint_a | Pubkey | 32 | Collateral mint (currency). |
| maturity_date | i64 | 8 | Maturity date. |
| creation_date | i64 | 8 | Creation date. |
| is_for_sale | bool | 1 | Flag for sale. |
| sale_price | u64 | 8 | Sale price (amount_b + interest). |
| sale_message | String | 132 | Sales message or bond description. |
| price_feed | Pubkey | 68 | Pyth feed id. |
| is_convertible | bool | 1 | Flag for conversion. |
| convertible | Convertible | 17 | Convertible condition. |

References:

- https://www.anchor-lang.com/docs/space
- https://solana.com/docs/core/accounts
