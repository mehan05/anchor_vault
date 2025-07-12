import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { clusterApiUrl, Connection, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { Vault } from "../target/types/vault";
import { expect } from "chai";

describe("vault", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.vault as Program<Vault>;

  const vault_state = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("state"), provider.wallet.publicKey.toBuffer()],
    program.programId
  )[0];

  const vault = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"),vault_state.toBuffer()],
    program.programId
  )[0];

  const getBalance = async (publicKey: PublicKey) => {
    return await provider.connection.getBalance(publicKey);
  };

  const numberToBn = (number: number) => {
    return new anchor.BN(number);
  };

  it("Initialize it", async () => {
    const tnx = await program.methods
      .initialize()
      .accountsPartial({
        signer: provider.wallet.publicKey,
        vaultState: vault_state,
        vault: vault,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Initialize Transaction:", tnx);

    const vaultStateAccountData = await program.account.vaultState.fetch(
      vault_state
    );

    expect(vaultStateAccountData.stateBump).to.be.a("number");
    expect(vaultStateAccountData.vaultBump).to.be.a("number");
  });

  it("deposit", async () => {
    try {
      
      const depositAmount = 0.5 * LAMPORTS_PER_SOL;
  
      const initialBalanceOfVault = await getBalance(vault);
  
      const tnx = await program.methods
        .deposit(numberToBn(depositAmount))
        .accountsPartial({
          signer: provider.wallet.publicKey,
          vaultState: vault_state,
          vault: vault,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
  
      console.log("Deposit Transaction:", tnx);
  
      const finalBalanceOfVault = await getBalance(vault);
  
      expect(finalBalanceOfVault - initialBalanceOfVault).to.equal(depositAmount);
    } catch (error) {
      console.log(error);
    }
  });

  it("withdraw", async () => {
    const withdrawAmount = 0.3 * LAMPORTS_PER_SOL;

    const initialBalanceOfVault = await getBalance(vault);
    console.log("initialBalanceOfVault",initialBalanceOfVault/LAMPORTS_PER_SOL);
    const tnx = await program.methods
      .withdraw(numberToBn(withdrawAmount))
      .accountsPartial({
        signer: provider.wallet.publicKey,
        vaultState: vault_state,
        vault: vault,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Withdraw Transaction:", tnx);

    const finalBalanceOfVault = await getBalance(vault);

    expect(finalBalanceOfVault + withdrawAmount).to.equal(
      initialBalanceOfVault
    );
  });

  it("close", async () => {
    const initialUserBalance = await getBalance(provider.wallet.publicKey);

    const vaultBalance = await getBalance(vault);

    const tnx = await program.methods
      .closeVault()
      .accountsPartial({
        signer: provider.wallet.publicKey,
        vaultState: vault_state,
        vault: vault,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Close Transaction:", tnx);

    const finalUserBalance = await getBalance(provider.wallet.publicKey);

    const finalvaultBalance = await getBalance(vault);

    expect(finalvaultBalance).to.equal(0);
    expect(finalUserBalance).to.greaterThan(initialUserBalance);

  });
});
