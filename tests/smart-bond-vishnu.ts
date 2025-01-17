import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { SmartBondVishnu } from "../target/types/smart_bond_vishnu";
import { randomBytes } from "crypto";
import {
  createAccount,
  createMint,
  mintTo,
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  AccountLayout,
  getAssociatedTokenAddressSync,
} from "@solana/spl-token";
import { LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";

describe("smart-bond-vishnu", () => {
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  anchor.setProvider(provider);
  const program = anchor.workspace.SmartBondVishnu as Program<SmartBondVishnu>;

  const issuer = anchor.web3.Keypair.generate();
  const owner = anchor.web3.Keypair.generate();
  const payer = (provider.wallet as NodeWallet).payer;
  const mint_a_authoriry = anchor.web3.Keypair.generate();
  const mint_b_authoriry = anchor.web3.Keypair.generate();

  const ammount_a = 5_000; // collateral (ETH)
  const ammount_b = 10_000_000; // loan (USDC)
  const ammount_c = 9_000_000; // sell price (USDC)
  const ammount_d = 1_000_000; // extra (USDC) to repay (1 + 9 = 10)

  let mint_a;
  let mint_b;
  let issuer_a_token;
  let issuer_b_token;
  let owner_a_token;
  let owner_b_token;
  let escrow: anchor.web3.PublicKey;
  let escrow_a_token: anchor.web3.PublicKey;

  const getRandomBigNumber = (size = 8) => {
    return new BN(randomBytes(size));
  };
  const addDays = (date, days) => {
    var result = new Date(date);
    result.setDate(result.getDate() + days);
    return result;
  };

  // https://docs.pyth.network/price-feeds/sponsored-feeds
  const PYTH_FEED_ID =
    "ef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";
  const PYTH_ACCOUNT_ADDRESS = new anchor.web3.PublicKey(
    "7UVimffxr9ow1uXYxsr4LHAcV58mLzhmwaeKvJ1pjLiE"
  );
  const BOND_ID = getRandomBigNumber();

  before(async () => {
    // Server responded with 429 Too Many Requests.
    await provider.connection.requestAirdrop(
      owner.publicKey,
      1 * LAMPORTS_PER_SOL
    );
    await provider.connection.requestAirdrop(
      payer.publicKey,
      1 * LAMPORTS_PER_SOL
    );
    await provider.connection.requestAirdrop(
      issuer.publicKey,
      1 * LAMPORTS_PER_SOL
    );

    console.log("Creating the 'A' (ETH) mint...");
    mint_a = await createMint(
      provider.connection,
      payer,
      mint_a_authoriry.publicKey,
      mint_a_authoriry.publicKey,
      9
    );

    console.log("Creating the 'B' (USDC) mint...");
    mint_b = await createMint(
      provider.connection,
      payer,
      mint_b_authoriry.publicKey,
      mint_b_authoriry.publicKey,
      9
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
      owner.publicKey
    );

    console.log("Creating owner 'B' token account...");
    owner_a_token = await createAccount(
      connection,
      payer,
      mint_a,
      owner.publicKey
    );

    console.log("Adding 1k (USDC) token for the owner...");
    await mintTo(
      connection,
      payer,
      mint_b,
      issuer_b_token,
      mint_b_authoriry,
      ammount_d,
      [],
      undefined,
      TOKEN_PROGRAM_ID
    );

    console.log("Adding 5 (ETH) token for the issuer...");
    await mintTo(
      connection,
      issuer,
      mint_a,
      issuer_a_token,
      mint_a_authoriry,
      ammount_a
    );

    console.log("Adding 10k (USDC) token for the owner...");
    await mintTo(
      connection,
      payer,
      mint_b,
      owner_b_token,
      mint_b_authoriry,
      ammount_b,
      [],
      undefined,
      TOKEN_PROGRAM_ID
    );

    // Derive escrow address
    [escrow] = await PublicKey.findProgramAddressSync(
      [Buffer.from("bond_account"), BOND_ID.toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    // Derive associated escrow address
    escrow_a_token = await getAssociatedTokenAddressSync(mint_a, escrow, true);
  });

  it("<Create the bond>", async () => {
    // https://www.anchor-lang.com/docs/javascript-anchor-types
    const maturityDate = new anchor.BN(addDays(new Date(), 30).getTime());
    const isForSale = true;
    const convertible = { whenGraterThan: { value: new anchor.BN(140) } };
    const ix = await program.methods
      .createBond(
        BOND_ID,
        "CryCo 24",
        new anchor.BN(ammount_a),
        new anchor.BN(ammount_b),
        maturityDate,
        isForSale,
        "High performance bond from CryCo24.",
        PYTH_FEED_ID,
        convertible
      )
      .accounts({
        issuer: issuer.publicKey,
        mintA: mint_a,
        mintB: mint_b,
        issuerAtaA: issuer_a_token,
        bondAccount: escrow,
        vaultAtaA: escrow_a_token,
        tokenProgram: TOKEN_PROGRAM_ID,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        systemProgram: SystemProgram.programId,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([issuer])
      .rpc({ skipPreflight: true });
  });

  it("<Find the bond>", async () => {
    const [userSmartBond, _] =
      await anchor.web3.PublicKey.findProgramAddressSync(
        [
          //anchor.utils.bytes.utf8.encode('bond_account'), payer.publicKey.toBuffer()
          Buffer.from("bond_account"),
          BOND_ID.toArrayLike(Buffer, "le", 8),
        ],
        program.programId
      );
    try {
      let bondDetails = await program.account.bondAccount.fetch(userSmartBond);
      console.log(
        ` Bond name :: ${bondDetails.name} \n Owner :: ${bondDetails.owner} \n Face mint :: ${bondDetails.mintB} \n Face amount :: ${bondDetails.amountB}`
      );

      const tokenAccount = await connection.getTokenAccountsByOwner(
        new PublicKey(escrow),
        { programId: TOKEN_PROGRAM_ID }
      );
      const accountData = AccountLayout.decode(
        tokenAccount.value[0].account.data
      );
      console.log(
        ` Collateral mint :: ${new PublicKey(
          accountData.mint
        )} \n Collateral amount :: ${accountData.amount}`
      );
      console.log(
        ` Mature date :: ${new Date(bondDetails.maturityDate.toNumber())}`
      );
    } catch (error) {
      console.log("Bond account does not exist :: ", error);
      throw error;
    }
  });

  it("<Sell the bond>", async () => {
    const tx = await program.methods
      .sellBond(
        true,
        new anchor.BN(ammount_c),
        `Hello, I am the new bond owner. I bought it for ${ammount_c}.`
      )
      .accounts({
        owner: issuer.publicKey, // issuer or owner
        bondAccount: escrow,
      })
      .signers([issuer]) // issuer or owner
      .rpc({ skipPreflight: true });
  });

  // Two scenarios are possible:
  // a) When the bond has been just issued, the owner is a new buyer (he is a signer).
  // b) When the bond already has an owner, buyer acts like a third party.
  it("<Buy the bond>", async () => {
    const tx = await program.methods
      .buyBond()
      .accounts({
        buyer: owner.publicKey, //buyer.publicKey,
        bondAccount: escrow,
        vaultAtaA: escrow_a_token,
        ownerAtaB: issuer_b_token, //owner_b_token,
        buyerAtaB: owner_b_token, //buyer_b_token,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([owner]) //([buyer])
      .rpc({ skipPreflight: true });
  });

  it("<Accounts revision>", async () => {
    await getMints("> Issuer (bond is sold)", issuer.publicKey);
    await getMints("> Owner (new owner)", owner.publicKey);
    await getMints("> Escrow", escrow);
  });

  it("<Check market price>", async () => {
    // This is another way how to import the feedAccount.
    // Now we use [[test.validator.clone]] in Anchor.toml.
    // const SOL_PRICE_FEED_ID = "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";
    // const solUsdPriceFeedAccount = pythSolanaReceiver.getPriceFeedAccountAddress(0, SOL_PRICE_FEED_ID).toBase58();
    // const solUsdPriceFeedAccountPubkey = new PublicKey(solUsdPriceFeedAccount);
    // const devnetConnection = new Connection("https://api.devnet.solana.com");
    // const feedAccountInfo = await devnetConnection.getAccountInfo(solUsdPriceFeedAccountPubkey);

    const tx = await program.methods
      .checkBond()
      .accounts({
        bondAccount: escrow,
        priceUpdate: PYTH_ACCOUNT_ADDRESS,
        clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc({ skipPreflight: true });

    console.log("Oracle Price Feed :: ", PYTH_ACCOUNT_ADDRESS.toString());
    console.log("Transaction signature :: ", tx);
  });

  it.skip("<Repay the bond>", async () => {
    const tx = await program.methods
      .repayBond()
      .accounts({
        issuer: issuer.publicKey,
        bondAccount: escrow,
        vaultAtaA: escrow_a_token,
        issuerAtaA: issuer_a_token,
        issuerAtaB: issuer_b_token,
        ownerAtaB: owner_b_token,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([issuer])
      .rpc({ skipPreflight: true });
  });

  it("<Convert the bond>", async () => {
    const tx = await program.methods
      .convertBond()
      .accounts({
        owner: owner.publicKey,
        bondAccount: escrow,
        vaultAtaA: escrow_a_token,
        ownerAtaA: owner_a_token,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([owner])
      .rpc({ skipPreflight: true });
  });

  it("<Accounts revision>", async () => {
    await getMints("> Issuer (bond is converted)", issuer.publicKey);
    await getMints("> Owner (ex holder)", owner.publicKey);
    await getMints("> Escrow", escrow);
  });

  it.skip("<Cancel the bond>", async () => {
    const tx = await program.methods
      .cancelBond()
      .accounts({
        issuer: issuer.publicKey,
        bondAccount: escrow,
        vaultAtaA: escrow_a_token,
        issuerAtaA: issuer_a_token,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([issuer])
      .rpc({ skipPreflight: false });
  });

  it.skip("<Accounts revision>", async () => {
    await getMints("> Issuer (bond is cancelled)", issuer.publicKey);
    await getMints("> Owner (new owner)", owner.publicKey);
    await getMints("> Escrow", escrow);
  });

  // https://spl.solana.com/token
  async function getMints(name: string, tokenAccountKey: PublicKey) {
    const tokenAccounts = await connection.getTokenAccountsByOwner(
      new PublicKey(tokenAccountKey),
      { programId: TOKEN_PROGRAM_ID }
    );

    console.log(`${name} token account`);
    tokenAccounts.value.forEach((tokenAccount) => {
      const accountData = AccountLayout.decode(tokenAccount.account.data);
      console.log(`${new PublicKey(accountData.mint)}   ${accountData.amount}`);
    });
    console.log("------------------------------------------------------------");
  }

  async function getBalances() {
    let payerBalance = await provider.connection.getBalance(payer.publicKey);
    console.log(` Payer account: ${payerBalance / LAMPORTS_PER_SOL}`);
    let issuerBalance = await provider.connection.getBalance(issuer.publicKey);
    console.log(` Issuer account: ${issuerBalance / LAMPORTS_PER_SOL}`);
    let ownerBalance = await provider.connection.getBalance(owner.publicKey);
    console.log(` Owner account: ${ownerBalance / LAMPORTS_PER_SOL}`);
    let bondBalance = await provider.connection.getBalance(escrow_a_token);
    console.log(` Bond account: ${bondBalance / LAMPORTS_PER_SOL}`);
  }
});
