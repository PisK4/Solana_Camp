import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Bob } from "../target/types/bob";
import { Alice } from "../target/types/alice";
import { expect } from "chai";

describe("CPI from Alice to Bob", () => {
  const provider = anchor.AnchorProvider.env();

  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  const aliceProgram = anchor.workspace.Alice as Program<Alice>;
  const bobProgram = anchor.workspace.Bob as Program<Bob>;

  const dataAccountKeypair = anchor.web3.Keypair.generate();
  let senderAccountPK;

  it("sender accont initializes", async () => {
    const senderAccoundSeeds = [Buffer.from("vizing_message_sender")];
    console.log("alice program id: ", aliceProgram.programId.toBase58());
    const [senderAccount, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      senderAccoundSeeds,
      aliceProgram.programId
    );
    senderAccountPK = senderAccount;
    console.log(`senderAccount: ${senderAccount.toBase58()}, bump: ${bump}`);
    console.log(
      "owner of pda before initialize:",
      await anchor.getProvider().connection.getAccountInfo(senderAccount)
    );
    const tx = await aliceProgram.methods
      .senderAccountInitializer(bobProgram.programId)
      .accounts({
        senderAccount: senderAccount,
      })
      .rpc();
    console.log(`initialize: ${tx}`);
    console.log(
      "owner of pda after initialize:",
      (
        await anchor.getProvider().connection.getAccountInfo(senderAccount)
      ).owner.toBase58()
    );

    // const senderAccoundSeeds = [Buffer.from("sender_seed")];
    // console.log("bob program id: ", bobProgram.programId.toBase58());
    // const [senderAccount, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    //   senderAccoundSeeds,
    //   bobProgram.programId
    // );
    // senderAccountPK = senderAccount;
    // console.log(`senderAccount: ${senderAccount.toBase58()}, bump: ${bump}`);
    // console.log(
    //   "owner of pda before initialize:",
    //   await anchor.getProvider().connection.getAccountInfo(senderAccount)
    // );
    // const tx = await bobProgram.methods
    //   .senderAccountInitializer(aliceProgram.programId)
    //   .accounts({
    //     senderAccount: senderAccount,
    //   })
    //   .rpc();
    // console.log(`initialize: ${tx}`);
    // console.log(
    //   "owner of pda after initialize:",
    //   (
    //     await anchor.getProvider().connection.getAccountInfo(senderAccount)
    //   ).owner.toBase58()
    // );
  });

  it("Bob initializes", async () => {
    // Add your test here.
    console.log("bob program id: ", bobProgram.programId.toBase58());
    const tx = await bobProgram.methods
      .initialize()
      .accounts({
        bobDataAccount: dataAccountKeypair.publicKey,
        signer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([dataAccountKeypair])
      .rpc();
    console.log(`initialize: ${tx}`);
  });

  it("Can add numbers then double!", async () => {
    console.log("alice program id: ", aliceProgram.programId.toBase58());
    // Add your test here.
    const tx = await aliceProgram.methods
      .askBobToAdd(new anchor.BN(4), new anchor.BN(2))
      .accounts({
        bobDataAccount: dataAccountKeypair.publicKey,
        senderAccount: senderAccountPK,
        bobProgram: bobProgram.programId,
      })
      .rpc();
    console.log(`askBobToAdd: ${tx}`);
  });

  it("Can assert value in Bob's data account equals 4 + 2", async () => {
    const BobAccountValue = (
      await bobProgram.account.bobData.fetch(dataAccountKeypair.publicKey)
    ).result.toNumber();
    expect(BobAccountValue).to.equal(6);
    console.log(`BobAccountValue: ${BobAccountValue}`);
  });
});
