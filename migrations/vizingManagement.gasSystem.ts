import * as anchor from "@coral-xyz/anchor";
import * as vizingUtils from "./utils/vizing.utils";
import * as vizingInit from "./utils/initial.vizingPad";
import initParams from "./vizingPad.InitParams.json";
import fs from "fs";

const vizingCoreProgramId = initParams.vizingPadProgramId;

const deployer: anchor.web3.PublicKey = new anchor.web3.PublicKey(
  initParams.deployer
);
const feeCollector: anchor.web3.PublicKey = new anchor.web3.PublicKey(
  initParams.feeCollector
);
const engineAdmin: anchor.web3.PublicKey = new anchor.web3.PublicKey(
  initParams.engineAdmin
);
const stationAdmin: anchor.web3.PublicKey = new anchor.web3.PublicKey(
  initParams.stationAdmin
);
const gasPoolAdmin: anchor.web3.PublicKey = new anchor.web3.PublicKey(
  initParams.gasPoolAdmin
);
const swapManager: anchor.web3.PublicKey = new anchor.web3.PublicKey(
  initParams.swapManager
);
const trustedRelayers: anchor.web3.PublicKey[] = initParams.trustedRelayers.map(
  (key) => new anchor.web3.PublicKey(key)
);
let registeredValidator: anchor.web3.PublicKey = new anchor.web3.PublicKey(
  initParams.registeredValidator
);

export async function main() {
  console.log("### vizing init start");

  console.log("initParams: ", initParams);

  let vizingProgram = vizingUtils.generateVizingPadProgram("devnet");

  if (deployer.toString() !== vizingProgram.provider.publicKey.toString()) {
    console.log(
      `left: ${deployer} != right: ${vizingProgram.provider.publicKey}`
    );
    throw new Error(
      "deployer keyPair wallet not match, set your keyPair wallet on Anhcor.toml file"
    );
  }

  if (vizingCoreProgramId !== vizingProgram.programId.toBase58()) {
    throw new Error(
      "vizingCoreProgramId not match, set your vizigCore Program KeyPair  on target/deploy/vizing_core-keypair.json file"
    );
  }

  const vizingPadProgramKeyPair =
    vizingUtils.loadProgramKeypairFromFile("vizingCore");

  if (
    vizingPadProgramKeyPair.publicKey.toString() !==
    vizingProgram.programId.toString()
  ) {
    console.log(
      `left: ${vizingPadProgramKeyPair.publicKey} != right: ${vizingProgram.programId}`
    );
    throw new Error(
      "vizingCoreProgramId not match, set your vizigCore Program KeyPair  on target/deploy/vizing_core-keypair.json file"
    );
  }

  console.log("start!");

  const gasSystemParams: vizingUtils.initializeVizingGasSystemParams = {
    groupId: new anchor.BN(initParams.firstSetup.groupId),
    chainId: new anchor.BN(initParams.firstSetup.chainId),
    basePrice: new anchor.BN(initParams.firstSetup.basePrice),
    molecular: new anchor.BN(initParams.firstSetup.molecular),
    denominator: new anchor.BN(initParams.firstSetup.denominator),
    molecularDecimal: initParams.firstSetup.molecularDecimal,
    denominatorDecimal: initParams.firstSetup.denominatorDecimal,
    globalBasePrice: new anchor.BN(initParams.firstSetup.globalBasePrice),
    defaultGasLimit: new anchor.BN(initParams.firstSetup.defaultGasLimit),
    amountInThreshold: new anchor.BN(initParams.firstSetup.amountInThreshold),
    globalMolecular: new anchor.BN(initParams.firstSetup.globalMolecular),
    globalDenominator: new anchor.BN(initParams.firstSetup.globalDenominator),
  };

  const [vizingPadConfigs, vizingPadConfigBump] =
    vizingUtils.generatePdaForVizingPadConfig(vizingProgram.programId);

  {
    const [vizingGasSystem, vizingGasSystemBump] =
      vizingUtils.generatePdaForVizingGasSystem(
        vizingProgram.programId,
        vizingPadConfigs,
        gasSystemParams.groupId
      );

    console.log("vizingGasSystem: ", vizingGasSystem.toString());

    const initGasSystemParams = gasSystemParams;

    {
      try {
        const tx = await vizingUtils.initializeVizingGasSystem(
          vizingProgram,
          initGasSystemParams,
          {
            vizingPadConfig: vizingPadConfigs,
            vizingGasSystem: vizingGasSystem,
            payer: vizingProgram.provider.publicKey,
          }
        );

        console.log(`gasSystem initialize: ${tx}`);
      } catch (error) {
        if (error.signature) {
          console.log(`gasSystem initialize signature: ${error.signature}`);
        } else {
          console.error(error);
        }
      }
    }
  }
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
