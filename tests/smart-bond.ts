import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SmartBond } from "../target/types/smart_bond";

describe("smart-bond", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.SmartBond as Program<SmartBond>;


  it("Creating a new bond account", async () => {
    const ix = await program.methods.createBond("CryCo 24", "Darius", "Darius", "USDC", 1000, "ETH", 5, "2024-12-31")
    const userSmartBond = (await ix.pubkeys()).bondAccount
    console.log("User bond account address :: ", userSmartBond.toString());
    // Create user's bond address
    const tx = await ix.rpc()
    console.log("Your transaction signature", tx);
    // Bond Details
    let bondDetails = await program.account.bondAccount.fetch(userSmartBond);
    console.log(`Created a new bond account with following details \n Bond name :: ${bondDetails.name} \n Owner :: ${bondDetails.owner} \n Currency :: ${bondDetails.currency} \n Amount :: ${bondDetails.amount}`)
  });


  it("Update bond owner", async () => {
    const ix = await program.methods.updateBondOwner("Mick")
    const userSmartBond = (await ix.pubkeys()).bondAccount
    console.log("User bond account address :: ", userSmartBond.toString());
    // Update user's bond owner
    const tx = await ix.rpc()
    console.log("Your transaction signature", tx);
    // Bond Details
    let bondDetails = await program.account.bondAccount.fetch(userSmartBond);
    console.log(`Bond account is updated \n Bond name :: ${bondDetails.name} \n Owner :: ${bondDetails.owner} \n Currency :: ${bondDetails.currency} \n Amount :: ${bondDetails.amount}`)
  });


  it("Find user's bond account", async () => {
    const [userSmartBond, _] = await anchor.web3.PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode('bond_account'),
        provider.wallet.publicKey.toBuffer(),
      ],
      program.programId
    )
    try {
      // Bond Details
      let bondDetails = await program.account.bondAccount.fetch(userSmartBond);
      console.log(`Bond account is found \n Bond name :: ${bondDetails.name} \n Owner :: ${bondDetails.owner} \n Currency :: ${bondDetails.currency} \n Amount :: ${bondDetails.amount}`)
    } catch (error) {
      console.log("Bond account does not exist :: ", error)
    }
  });


  it("Delete bond account", async () => {
    const ix = await program.methods.deleteBond()
    const userSmartBond = (await ix.pubkeys()).bondAccount
    console.log("User bond account address :: ", userSmartBond.toString());
    // Delete user's bond address
    const tx = await ix.rpc()
    console.log("Your transaction signature", tx);
    try {
      let bondDetails = await program.account.bondAccount.fetch(userSmartBond);
      console.log(`Bond account is found \n Bond name :: ${bondDetails.name} \n Owner :: ${bondDetails.owner} \n Currency :: ${bondDetails.currency} \n Amount :: ${bondDetails.amount}`)
    } catch {
      console.log("Bond account is not found, the account was closed");
    }
  });

});