import * as anchor from "@coral-xyz/anchor";
import * as vizingUtils from "./vizing.utils";
import fs from "fs";

const deployerKeyPairPath = "governance/.config/wallet/deployer_keypair.json";

let vizingPadConfigs: anchor.web3.PublicKey;
let vizingAuthority: anchor.web3.PublicKey;
let vizingAppConfig: anchor.web3.PublicKey;
let vizingFeeRouter: anchor.web3.PublicKey;
let vizingMessageAuthority: anchor.web3.PublicKey;
let vizingGasSystem: anchor.web3.PublicKey;

export async function inititalizeVizingPad(
  vizingProgram: anchor.Program,
  deployerPk: anchor.web3.PublicKey,
  feeCollector: anchor.web3.PublicKey,
  engineAdmin: anchor.web3.PublicKey,
  stationAdmin: anchor.web3.PublicKey,
  gasPoolAdmin: anchor.web3.PublicKey,
  swapManager: anchor.web3.PublicKey,
  trustedRelayers: anchor.web3.PublicKey[],
  registeredValidator: anchor.web3.PublicKey,
  gasSystemParams: vizingUtils.initializeVizingGasSystemParams
) {
  console.log("### inititializeVizingPad start");

  const vizingPadInitParams: vizingUtils.initializeVizingPadParams = {
    owner: deployerPk,
    feeCollector: feeCollector,
    engineAdmin: engineAdmin,
    gasPoolAdmin: gasPoolAdmin,
    swapManager: swapManager,
    stationAdmin: stationAdmin,
    trustedRelayers: trustedRelayers,
    registeredValidator: registeredValidator,
    relayers: trustedRelayers,
    isPaused: false,
  };

  let vizingPadConfigBump: number;

  {
    console.log("initializing vizingPadConfigs pda");
    const [vizingPad, bump] = vizingUtils.generatePdaForVizingPadConfig(
      vizingProgram.programId
    );

    vizingPadConfigs = vizingPad;
    vizingPadConfigBump = bump;

    console.log(`vizingPad: ${vizingPad.toBase58()}, bump: ${bump}`);
  }

  {
    console.log("initializing vizingAuthority pda");
    const [authority, bump] = vizingUtils.generatePdaForVizingAuthority(
      vizingProgram.programId,
      vizingPadConfigs
    );

    const authorityU8Array = new Uint8Array(authority.toBuffer().slice(0, 32));

    vizingAuthority = authority;
    console.log(`authority: ${authority.toBase58()}, bump: ${bump}`);

    // console.log("authorityU8Array:", authorityU8Array);
  }

  {
    const tx = await vizingUtils.initializeVizingPad(
      vizingProgram,
      vizingPadInitParams,
      {
        vizingPadConfig: vizingPadConfigs,
        vizingPadAuthority: vizingAuthority,
        payer: deployerPk,
      }
    );

    console.log(`vizingPad initialize: ${tx}`);
  }

  {
    console.log("initializing vizingGasSystem pda");
    const [gasSys, bump] = vizingUtils.generatePdaForVizingGasSystem(
      vizingProgram.programId,
      vizingPadConfigs
    );

    vizingGasSystem = gasSys;

    console.log(
      `vizingGasSystem: ${vizingGasSystem.toBase58()}, bump: ${bump}`
    );

    const initGasSystemParams = gasSystemParams;

    {
      const tx = await vizingUtils.initializeVizingGasSystem(
        vizingProgram,
        initGasSystemParams,
        {
          vizingPadConfig: vizingPadConfigs,
          vizingGasSystem: vizingGasSystem,
          payer: deployerPk,
        }
      );

      console.log(`gasSystem initialize: ${tx}`);
    }
  }

  {
    const [recordMessageAuthority, recordMessageBump] =
      vizingUtils.generatePdaForRecordMessage(vizingProgram.programId);
    {
      const tx = await vizingUtils.initializeRecordMessage(
        vizingProgram,
        recordMessageAuthority,
        deployerPk
      );
      console.log(`recordMessage initialize: ${tx}`);
    }
  }

  return {
    vizingPadConfigs,
    vizingPadConfigBump,
    vizingPadInitParams,
    vizingAuthority,
    vizingGasSystem,
  };
}

export async function inititalizeRegisterVizingApp(
  vizingPadProgram: anchor.Program,
  deployerPk: anchor.web3.PublicKey,
  vizingAppProgramId: anchor.web3.PublicKey,
  vizingAppAccounts: anchor.web3.PublicKey[]
) {
  console.log("### inititalizeRegisterVizingApp start");

  {
    const [solReceiver, bump1] =
      vizingUtils.generatePdaForVizingAppSolReceiver(vizingAppProgramId);
    vizingFeeRouter = solReceiver;

    console.log(`solPdaReceiver: ${solReceiver.toBase58()}`);
  }

  {
    // #### register vizing app start
    const [vizingAppContract, vizingAppBump] =
      vizingUtils.generatePdaForVizingAppConfig(
        vizingPadProgram.programId,
        vizingAppProgramId
      );

    console.log(
      `vizingAppConfig: ${vizingAppContract.toBase58()}, bump: ${vizingAppBump}`
    );

    vizingAppConfig = vizingAppContract;

    const registerParams = {
      solPdaReceiver: vizingFeeRouter,
      vizingAppAccounts: vizingAppAccounts,
      vizingAppProgramId: vizingAppProgramId,
    };

    const tx = await vizingUtils.vizingAppRegister(
      vizingPadProgram,
      registerParams,
      deployerPk,
      vizingAppConfig
    );

    console.log(`register vizing app: ${tx}`);
  }

  return {
    vizingFeeRouter,
    vizingAppConfig,
  };
}

export async function initializeVizingApp(
  vizingAppProgram: anchor.Program,
  deployerPk: anchor.web3.PublicKey
) {
  const vizingAppProgramId = vizingAppProgram.programId;
  console.log(`### initializeVizingApp start ${vizingAppProgramId}`);
  {
    const [messageAuthority, bump3] =
      anchor.web3.PublicKey.findProgramAddressSync(
        [vizingUtils.vizingMessageAuthoritySeed],
        vizingAppProgramId
      );

    vizingMessageAuthority = messageAuthority;

    console.log(
      `messageAuthority: ${messageAuthority.toBase58()}, bump: ${bump3}`
    );

    await vizingAppProgram.methods
      .initializeVizingEmitter()
      .accounts({
        messagePdaAuthority: vizingMessageAuthority,
        payer: deployerPk,
      })
      .rpc();
  }

  return {
    vizingMessageAuthority,
  };
}

export function loadKeypairFromFile(filename: string): anchor.web3.Keypair {
  const secret = JSON.parse(fs.readFileSync(filename).toString()) as number[];
  const secretKey = Uint8Array.from(secret);
  return anchor.web3.Keypair.fromSecretKey(secretKey);
}
