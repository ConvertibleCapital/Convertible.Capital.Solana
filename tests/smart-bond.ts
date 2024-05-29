import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SmartBond } from "../target/types/smart_bond";
import { randomBytes } from "crypto";
import { createAccount, createMint, mintTo, TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, AccountLayout, getOrCreateAssociatedTokenAccount, getAccount } from "@solana/spl-token";
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY, Transaction, TransactionMessage, VersionedTransaction } from "@solana/web3.js";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";

describe("smart-bond", () => {
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  anchor.setProvider(provider);
  const program = anchor.workspace.SmartBond as Program<SmartBond>;

  const issuer = anchor.web3.Keypair.generate();
  const owner = anchor.web3.Keypair.generate();
  const payer = (provider.wallet as NodeWallet).payer;
  const escrowedXTokens = anchor.web3.Keypair.generate();

  const ammount_a = 5_000;        // collateral (ETH)
  const ammount_b = 10_000_000;   // loan (USDC)

  let mint_a;
  let mint_b;
  let issuer_a_token;
  let issuer_b_token;
  let owner_a_token;
  let owner_b_token;
  let escrow: anchor.web3.PublicKey;

  before(async () => {

    // Server responded with 429 Too Many Requests.
    await provider.connection.requestAirdrop(owner.publicKey, 1 * LAMPORTS_PER_SOL);
    await provider.connection.requestAirdrop(payer.publicKey, 1 * LAMPORTS_PER_SOL);
    await provider.connection.requestAirdrop(issuer.publicKey, 1 * LAMPORTS_PER_SOL);

    // Derive escrow address
    [escrow] = PublicKey.findProgramAddressSync(
      [Buffer.from("bond_account"), issuer.publicKey.toBuffer()],
      program.programId
    )

    console.log("Creating the 'A' (ETH) mint...");
    mint_a = await createMint(
      provider.connection,
      payer,
      issuer.publicKey,
      issuer.publicKey,
      6
    );

    console.log("Creating the 'B' (USDC) mint...");
    mint_b = await createMint(
      provider.connection,
      payer,
      owner.publicKey,
      owner.publicKey,
      6
    );

    // issurer (Seller)
    console.log("Creating issuer 'A' token account...");
    issuer_a_token = await createAccount(
      connection,
      payer,
      mint_a,
      issuer.publicKey
    );

    console.log("Creating issuer 'B' token account...");
    issuer_b_token = await createAccount(
      connection,
      payer,
      mint_b,
      issuer.publicKey
    );

    // owner (Buyer)
    console.log("Creating owner 'A' token account...");
    owner_b_token = await createAccount(
      connection,
      payer,
      mint_b,
      owner.publicKey,
    );

    console.log("Creating owner 'B' token account...");
    owner_a_token = await createAccount(
      connection,
      payer,
      mint_a,
      owner.publicKey,
    );

    console.log("Adding 5 (ETH) token for the issuer...");
    await mintTo(
      connection,
      payer,
      mint_a,
      issuer_a_token,
      issuer,
      ammount_a,
      [],
      undefined,
      TOKEN_PROGRAM_ID,
    );

    console.log("Adding 10k (USDC) token for the owner...");
    const mintSig = await mintTo(
      connection,
      payer,
      mint_b,
      owner_b_token,
      owner,
      ammount_b,
      [],
      undefined,
      TOKEN_PROGRAM_ID,
    );
  })


  it("<Create the bond>", async () => {
    const seed = new anchor.BN(randomBytes(8));
    const ix = await program.methods
      .initialize(seed, "CryCo 24", new anchor.BN(ammount_a), new anchor.BN(ammount_b), "2024-12-31")
      .accounts(
        {
          issuer: issuer.publicKey,
          issuerMintA: mint_a,
          issuerMintB: mint_b,
          issuerAtaA: issuer_a_token,
          bondAccount: escrow,
          vaultAccount: escrowedXTokens.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId
        }
      )
      .signers([issuer, escrowedXTokens])
      .rpc({ skipPreflight: true });
  });


  // it("Update bond owner", async () => {
  //   const ix = await program.methods.updateBondOwner("Mick")
  //   const userSmartBond = (await ix.pubkeys()).bondAccount
  //   console.log("User bond account address :: ", userSmartBond.toString());
  //   // Update user's bond owner
  //   const tx = await ix.rpc()
  //   console.log("Your transaction signature", tx);
  //   // Bond Details
  //   let bondDetails = await program.account.bondAccount.fetch(userSmartBond);
  //   console.log(`Bond account is updated \n Bond name :: ${bondDetails.name} \n Owner :: ${bondDetails.owner} \n Currency :: ${bondDetails.currency} \n Amount :: ${bondDetails.amount}`)
  // });


  it("<Find the bond>", async () => {
    const [userSmartBond, _] = await anchor.web3.PublicKey.findProgramAddressSync(
      [
        //anchor.utils.bytes.utf8.encode('bond_account'), payer.publicKey.toBuffer()
        Buffer.from("bond_account"), issuer.publicKey.toBuffer()
      ],
      program.programId
    )
    try {
      let bondDetails = await program.account.bondAccount.fetch(userSmartBond);
      console.log(` Bond name :: ${bondDetails.name} \n Owner :: ${bondDetails.owner} \n Face mint :: ${bondDetails.mintB} \n Face amount :: ${bondDetails.amountB}`)

      const tokenAccount = await connection.getTokenAccountsByOwner(new PublicKey(escrow), { programId: TOKEN_PROGRAM_ID });
      const accountData = AccountLayout.decode(tokenAccount.value[0].account.data);
      console.log(` Collateral mint :: ${new PublicKey(accountData.mint)} \n Collateral amount :: ${accountData.amount}`);

    } catch (error) {
      console.log("Bond account does not exist :: ", error)
      throw error
    }
  });

  it("<Buy the bond>", async () => {
    const tx = await program.methods.buy()
      .accounts({
        owner: owner.publicKey,
        bondAccount: escrow,
        vaultAccount: escrowedXTokens.publicKey,
        issuerAtaB: issuer_b_token,
        ownerAtaA: owner_a_token,
        ownerAtaB: owner_b_token,
        tokenProgram: TOKEN_PROGRAM_ID
      })
      .signers([owner])
      .rpc();
  });

  it("<Accounts revision>", async () => {
    //const [userSmartBond, _] = await anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("bond_account"), seller.publicKey.toBuffer()], program.programId);
    //let bondDetails = await program.account.bondAccount.fetch(userSmartBond);
    //console.log(` New Bond owner :: ${bondDetails.owner} `);
    await getMints('> Issuer (bond is sold)', issuer.publicKey);
    await getMints('> Owner (new owner)', owner.publicKey);
    await getMints('> Escrow', escrow);
  });

  it("<Check market price>", async () => {
    const PYTH_FEED_ID = new anchor.web3.PublicKey(
      "H6ARHf6YXhGYeQfUzQNGk6rDNnLBQKrenN712K4AQJEG"
    );
    const tx = await program.methods
      .price()
      .accounts({
        priceFeed: PYTH_FEED_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
      })
      .rpc({ skipPreflight: true });
    console.log("Oracle Price Feed :: ", PYTH_FEED_ID.toString())
    console.log("Transaction signature :: ", tx);
  });

  it("<Convert the bond>", async () => {
    const tx = await program.methods.convert()
      .accounts({
        owner: owner.publicKey,
        bondAccount: escrow,
        vaultAccount: escrowedXTokens.publicKey,
        issuerAtaB: issuer_b_token,
        ownerAtaA: owner_a_token,
        ownerAtaB: owner_b_token,
        tokenProgram: TOKEN_PROGRAM_ID
      })
      .signers([owner])
      .rpc();
  });

  it("<Accounts revision>", async () => {
    await getMints('> Issuer (bond is sold)', issuer.publicKey);
    await getMints('> Owner (new owner)', owner.publicKey);
    await getMints('> Escrow', escrow);
  });

  // https://spl.solana.com/token
  async function getMints(name: string, tokenAccountKey: PublicKey) {
    const tokenAccounts = await connection.getTokenAccountsByOwner(
      new PublicKey(tokenAccountKey), { programId: TOKEN_PROGRAM_ID });

    console.log(`${name} token account`);
    tokenAccounts.value.forEach((tokenAccount) => {
      const accountData = AccountLayout.decode(tokenAccount.account.data);
      console.log(`${new PublicKey(accountData.mint)}   ${accountData.amount}`);
    })
    console.log("------------------------------------------------------------");
  }

  async function getBalances() {
    let payerBalance = await provider.connection.getBalance(payer.publicKey)
    console.log(` Payer account: ${payerBalance / LAMPORTS_PER_SOL}`)
    let issuerBalance = await provider.connection.getBalance(issuer.publicKey)
    console.log(` Issuer account: ${issuerBalance / LAMPORTS_PER_SOL}`)
    let ownerBalance = await provider.connection.getBalance(owner.publicKey)
    console.log(` Owner account: ${ownerBalance / LAMPORTS_PER_SOL}`)
    let bondBalance = await provider.connection.getBalance(escrowedXTokens.publicKey)
    console.log(` Bond account: ${bondBalance / LAMPORTS_PER_SOL}`)
  }


  // it("Delete bond account", async () => {
  //   const ix = await program.methods.deleteBond()
  //   const userSmartBond = (await ix.pubkeys()).bondAccount
  //   console.log("User bond account address :: ", userSmartBond.toString());
  //   // Delete user's bond address
  //   const tx = await ix.rpc()
  //   console.log("Your transaction signature", tx);
  //   try {
  //     let bondDetails = await program.account.bondAccount.fetch(userSmartBond);
  //     console.log(`Bond account is found \n Bond name :: ${bondDetails.name} \n Owner :: ${bondDetails.owner} \n Currency :: ${bondDetails.currency} \n Amount :: ${bondDetails.amount}`)
  //   } catch {
  //     console.log("Bond account is not found, the account was closed");
  //   }
  // });

});
