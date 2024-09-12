import * as anchor from "@coral-xyz/anchor";
import { Program } from "@project-serum/anchor";
import { VizingCore } from "../target/types/vizing_core";

export async function main() {
  console.log("### Deploy start");

  const wallet = anchor.AnchorProvider.local().wallet;

  const connection = new anchor.web3.Connection(
    "https://rpc.ankr.com/solana_devnet",
    "confirmed"
  );

  const provider = new anchor.AnchorProvider(connection, wallet, {
    commitment: "confirmed",
  });

  anchor.setProvider(provider);

  const vizingProgram = anchor.workspace.Alice as Program<VizingCore>;

  console.log("vizingProgram.programId:", vizingProgram.programId);
}

main()
  .catch((error) => {
    console.error(error);
    process.exitCode = 1;
  })
  .finally(() => {
    // exit the script
    process.exit();
  });
