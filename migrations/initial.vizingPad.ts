import * as anchor from "@coral-xyz/anchor";
import * as vizingUtils from "./vizing.utils";
import fs from "fs";

const deployerKeyPairPath = "governance/.config/wallet/deployer_keypair.json";

let vizingPadConfigs: anchor.web3.PublicKey;
let vizingAuthority: anchor.web3.PublicKey;
let vizingAppConfig: anchor.web3.PublicKey;
let solPdaReceiver: anchor.web3.PublicKey;
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
  let vizingAuthorityBump: number;
  let vizingGasSystemBump: number;
  let recordMessageAuthority: anchor.web3.PublicKey;
  let recordMessagesBump: number;

  {
    console.log("initializing vizingPadConfigs pda");
    [vizingPadConfigs, vizingPadConfigBump] =
      vizingUtils.generatePdaForVizingPadConfig(vizingProgram.programId);

    // vizingPadConfigs = vizingPad;
    // vizingPadConfigBump = bump;

    // console.log(`vizingPad: ${vizingPad.toBase58()}, bump: ${bump}`);
  }

  {
    console.log("initializing vizingAuthority pda");
    [vizingAuthority, vizingAuthorityBump] =
      vizingUtils.generatePdaForVizingAuthority(
        vizingProgram.programId,
        vizingPadConfigs
      );

    const authorityU8Array = new Uint8Array(
      vizingAuthority.toBuffer().slice(0, 32)
    );

    // vizingAuthority = authority;
    // console.log(`authority: ${authority.toBase58()}, bump: ${bump}`);

    // console.log("authorityU8Array:", authorityU8Array);
  }

  {
    try {
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
    } catch (error) {
      if (error.signature) {
        console.log(`signature: ${error.signature}`);
      } else {
        console.error(error);
      }
    }
  }

  {
    console.log("initializing vizingGasSystem pda");
    [vizingGasSystem, vizingGasSystemBump] =
      vizingUtils.generatePdaForVizingGasSystem(
        vizingProgram.programId,
        vizingPadConfigs
      );

    const initGasSystemParams = gasSystemParams;

    {
      try {
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
      } catch (error) {
        if (error.signature) {
          console.log(`signature: ${error.signature}`);
        } else {
          console.error(error);
        }
      }
    }
  }

  {
    [recordMessageAuthority, recordMessagesBump] =
      vizingUtils.generatePdaForRecordMessage(vizingProgram.programId);
    {
      try {
        const tx = await vizingUtils.initializeRecordMessage(
          vizingProgram,
          recordMessageAuthority,
          deployerPk
        );
        console.log(`recordMessage initialize: ${tx}`);
      } catch (error) {
        if (error.signature) {
          console.log(`signature: ${error.signature}`);
        } else {
          console.error(error);
        }
      }
    }
  }

  const ret = {
    vizingPadConfigs,
    vizingPadConfigBump,
    vizingAuthority,
    vizingAuthorityBump,
    vizingGasSystem,
    vizingGasSystemBump,
    recordMessageAuthority,
    recordMessagesBump,
  };

  console.table(vizingUtils.formatReturnInfo(ret));

  return {
    ...ret,
    vizingPadInitParams,
  };
}

export async function inititalizeRegisterVizingApp(
  vizingPadProgram: anchor.Program,
  deployerPk: anchor.web3.PublicKey,
  vizingAppProgramId: anchor.web3.PublicKey,
  vizingAppAccounts: anchor.web3.PublicKey[]
) {
  console.log("### inititalizeRegisterVizingApp start");
  let vizingAppBump: number;

  // #### register vizing app start
  [vizingAppConfig, vizingAppBump] = vizingUtils.generatePdaForVizingAppConfig(
    vizingPadProgram.programId,
    vizingAppProgramId
  );

  const registerParams = {
    solPdaReceiver: solPdaReceiver,
    vizingAppAccounts: vizingAppAccounts,
    vizingAppProgramId: vizingAppProgramId,
  };

  const ret = {
    vizingAppConfig,
  };

  console.table(vizingUtils.formatReturnInfo(ret));

  try {
    const tx = await vizingUtils.vizingAppRegister(
      vizingPadProgram,
      registerParams,
      deployerPk,
      vizingAppConfig
    );

    console.log(`register vizing app: ${tx}`);
  } catch (error) {
    if (error.signature) {
      console.log(`signature: ${error.signature}`);
    } else {
      console.error(error);
    }
  }

  return ret;
}

export async function initializeVizingApp(
  vizingAppProgram: anchor.Program,
  deployerPk: anchor.web3.PublicKey
) {
  const vizingAppProgramId = vizingAppProgram.programId;
  console.log(`### initializeVizingApp start ${vizingAppProgramId}`);
  let messageAuthorityBump: number;
  let solPdaReceiverBump: number;
  let vizingAppAuthority: anchor.web3.PublicKey;
  [vizingAppAuthority, messageAuthorityBump] =
    anchor.web3.PublicKey.findProgramAddressSync(
      [vizingUtils.vizingMessageAuthoritySeed],
      vizingAppProgramId
    );

  const vizingAppAuthorityHex = Buffer.from(
    vizingAppAuthority.toBytes()
  ).toString("hex");

  try {
    await vizingAppProgram.methods
      .initializeVizingEmitter()
      .accounts({
        messagePdaAuthority: vizingAppAuthority,
        payer: deployerPk,
      })
      .rpc();
  } catch (error) {
    console.error(error);
  }

  {
    [solPdaReceiver, solPdaReceiverBump] =
      vizingUtils.generatePdaForVizingAppSolReceiver(vizingAppProgramId);
  }

  try {
    await vizingAppProgram.methods
      .initializeVizingReceiver()
      .accounts({
        solPdaReceiver: solPdaReceiver,
        payer: deployerPk,
      })
      .rpc();
  } catch (error) {
    console.error(error);
  }

  const vizingAppProgramHex = Buffer.from(
    vizingAppProgram.programId.toBytes()
  ).toString("hex");

  const ret = {
    vizingAppProgramId,
    vizingAppProgramHex,
    vizingAppAuthority,
    vizingAppAuthorityHex,
    messageAuthorityBump,
    solPdaReceiver,
    solPdaReceiverBump,
  };

  console.table(vizingUtils.formatReturnInfo(ret));

  return ret;
}

export async function initializeVizingAppMock(
  vizingAppProgram: anchor.Program,
  deployerPk: anchor.web3.PublicKey
) {
  let resultDataAccount: anchor.web3.PublicKey;
  let resultDataBump: number;
  [resultDataAccount, resultDataBump] = vizingUtils.generatePdaForResultData(
    vizingAppProgram.programId
  );

  const ret = {
    resultDataAccount,
    resultDataBump,
  };

  console.table(vizingUtils.formatReturnInfo(ret));

  await vizingAppProgram.methods
    .initialize()
    .accounts({
      resultAccount: resultDataAccount,
      payer: deployerPk,
    })
    .rpc();

  return ret;
}

export function loadKeypairFromFile(filename: string): anchor.web3.Keypair {
  const secret = JSON.parse(fs.readFileSync(filename).toString()) as number[];
  const secretKey = Uint8Array.from(secret);
  return anchor.web3.Keypair.fromSecretKey(secretKey);
}
