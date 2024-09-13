import * as anchor from "@coral-xyz/anchor";
import { Program } from "@project-serum/anchor";
import { VizingCore } from "../target/types/vizing_core";
import fs from "fs";

const VizingPadConfigsSeed = Buffer.from("Vizing_Pad_Settings_Seed");
const vizingAuthoritySeed = Buffer.from("Vizing_Authority_Seed");
const vizingAppConfigSeed = Buffer.from("Vizing_App_Config_Seed");
const vizingAppSolReceiverSeed = Buffer.from("Vizing_App_Sol_Receiver_Seed");
const vizingFeeRouterSeed = Buffer.from("Vizing_Fee_Router_Seed");
const vizingMessageAuthoritySeed = Buffer.from("Vizing_Message_Authority_Seed");
const vizingGasSystemSeed = Buffer.from("init_mapping_fee_config");

const deployerKeyPairPath = "governance/.config/wallet/deployer_keypair.json";

let vizingPadConfigs: anchor.web3.PublicKey;
let vizingAuthority: anchor.web3.PublicKey;
let vizingAppConfig: anchor.web3.PublicKey;
let vizingFeeRouter: anchor.web3.PublicKey;
let vizingMessageAuthority: anchor.web3.PublicKey;
let vizingGasSystem: anchor.web3.PublicKey;

let deployer: anchor.web3.Keypair;
let feeCollector: anchor.web3.PublicKey;
let engineAdmin: anchor.web3.PublicKey;
let stationAdmin: anchor.web3.PublicKey;
let gasPoolAdmin: anchor.web3.PublicKey;
let trustedRelayers: anchor.web3.PublicKey[];
let registeredValidators: anchor.web3.PublicKey[];

export async function main() {
  console.log("### Deploy start");

  const wallet = anchor.AnchorProvider.local().wallet;
  deployer = loadKeypairFromFile(deployerKeyPairPath);
  feeCollector = engineAdmin = stationAdmin = gasPoolAdmin = deployer.publicKey;

  console.log(`deployer: ${deployer.publicKey.toBase58()}`);

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

  // const balance = await connection.getBalance(provider.wallet.publicKey);
  // console.log("balance:", balance);

  // initialize
  {
    console.log("start initialize");
    let vizingPadBump: number;
    let relayerBump: number;
    {
      const seed = [VizingPadConfigsSeed];
      const [vizingPad, bump] = anchor.web3.PublicKey.findProgramAddressSync(
        seed,
        vizingProgram.programId
      );

      vizingPadConfigs = vizingPad;
      vizingPadBump = bump;

      console.log(`vizingPad: ${vizingPad.toBase58()}, bump: ${bump}`);
    }

    const initParams = {
      owner: provider.wallet.publicKey,
      feeCollector: feeCollector,
      engineAdmin: engineAdmin,
      gasPoolAdmin: gasPoolAdmin,
      stationAdmin: stationAdmin,
      trustedRelayers: trustedRelayers.map((pubkey) => pubkey),
      registeredValidator: registeredValidators.map((pubkey) => pubkey),
      relayers: trustedRelayers.map((pubkey) => pubkey),
      isPaused: false,
    };

    {
      const seed = [vizingAuthoritySeed];
      const [authority, bump] = anchor.web3.PublicKey.findProgramAddressSync(
        seed,
        vizingProgram.programId
      );

      vizingAuthority = authority;
      console.log(`authority: ${authority.toBase58()}, bump: ${bump}`);
    }

    {
      const tx = await vizingProgram.methods
        .initializeVizingPad(initParams)
        .accounts({
          vizingPadConfig: vizingPadConfigs,
          vizingPadAuthority: vizingAuthority,
          payer: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
      console.log(`initialize: ${tx}`);
    }

    {
      const seed = [vizingGasSystemSeed];
      const [gasSys, bump] = anchor.web3.PublicKey.findProgramAddressSync(
        seed,
        vizingProgram.programId
      );

      vizingGasSystem = gasSys;

      console.log(
        `vizingGasSystem: ${vizingGasSystem.toBase58()}, bump: ${bump}`
      );
      const id = new anchor.BN(28516);
      let basePrice = new anchor.BN(anchor.web3.LAMPORTS_PER_SOL);
      let reserve = new anchor.BN(0);
      let molecular = new anchor.BN(0);
      let denominator = new anchor.BN(10);
      let molecular_decimal = 1;
      let denominator_decimal = 1;

      await vizingProgram.methods
        .initFeeConfig(
          id,
          basePrice,
          reserve,
          molecular,
          denominator,
          molecular_decimal,
          denominator_decimal
        )
        .accounts({
          mappingFeeConfig: vizingGasSystem,
          user: provider.wallet.publicKey,
        })
        .rpc();

      let new_global_base_price = new anchor.BN(anchor.web3.LAMPORTS_PER_SOL);
      let new_default_gas_limit = new anchor.BN(1);
      let new_amount_in_threshold = new anchor.BN(
        anchor.web3.LAMPORTS_PER_SOL * 100
      );
      await vizingProgram.methods
        .setThisGasGlobal(
          id,
          new_global_base_price,
          new_default_gas_limit,
          new_amount_in_threshold,
          molecular,
          denominator
        )
        .accounts({
          mappingFeeConfig: vizingGasSystem,
          vizing: vizingPadConfigs,
          user: gasPoolAdmin,
        })
        .rpc();
    }

    {
      const [recordMessageAuthority, recordMessageBump] =
        anchor.web3.PublicKey.findProgramAddressSync(
          [Buffer.from("init_current_record_message")],
          vizingProgram.programId
        );

      await vizingProgram.methods
        .initRecordMessage()
        .accounts({
          currentRecordMessage: recordMessageAuthority,
          user: deployer.publicKey,
        })
        .signers([])
        .rpc();
    }
  }
}

export function loadKeypairFromFile(filename: string): anchor.web3.Keypair {
  const secret = JSON.parse(fs.readFileSync(filename).toString()) as number[];
  const secretKey = Uint8Array.from(secret);
  return anchor.web3.Keypair.fromSecretKey(secretKey);
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
