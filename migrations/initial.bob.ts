import * as anchor from "@coral-xyz/anchor";
import { Wallet, AnchorProvider, Program } from "@project-serum/anchor";
import { Alice } from "../target/types/alice";
import { Bob } from "../target/types/bob";

describe("Vizing Deploy", () => {
  console.log("### Deploy start");

  const wallet = anchor.AnchorProvider.local().wallet;

  const connection = new anchor.web3.Connection(
    "https://rpc.ankr.com/solana_devnet",
    "confirmed"
  );

  // const connection = new anchor.web3.Connection(
  //   anchor.web3.clusterApiUrl("devnet"),
  //   "confirmed"
  // );

  // const connection = new anchor.web3.Connection(
  //   "https://solana-devnet.g.alchemy.com/v2/-m2gJ2Fiv4w403IMR27nGoHUyonc0azl",
  //   "confirmed"
  // );

  // const provider = new anchor.AnchorProvider(connection, wallet, {
  //   commitment: "confirmed",
  // });

  const provider = anchor.AnchorProvider.env();

  anchor.setProvider(provider);

  const aliceProgram = anchor.workspace.Alice as Program<Alice>;
  const bobProgram = anchor.workspace.Bob as Program<Bob>;

  it.skip("Initializes Alice", async () => {
    const keypair = anchor.web3.Keypair.generate();
    const tx = await aliceProgram.methods
      .initialize()
      .accounts({
        signer: keypair.publicKey,
      })
      .signers([keypair])
      .rpc();
    console.log(`initialize: ${tx}`);
  });

  it.skip("Initializes Bob", async () => {
    const keypair = anchor.web3.Keypair.generate();
    console.log("keypair:", keypair);
    console.log("keypair.publicKey:", keypair.publicKey.toBase58());
    {
      const tx = await bobProgram.methods
        .initialize()
        .accounts({
          bobDataAccount: keypair.publicKey,
          signer: provider.wallet.publicKey,
        })
        .signers([keypair])
        .rpc();
      console.log(`bob initialize: ${tx}`);
    }
  });

  it("check balance Bob", async () => {
    const balance = await connection.getBalance(provider.wallet.publicKey);
    console.log("balance:", balance);

    const bobBalance = await connection.getBalance(bobProgram.programId);
    console.log("bobBalance:", bobBalance);
  });
});
