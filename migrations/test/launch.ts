import * as anchor from "@coral-xyz/anchor";
import * as vizingUtils from "../utils/vizing.utils";
import devnetConfig from "../deployment.devnet.json";

export async function main() {
  console.log("### Launch start");
  const vizingProgram = vizingUtils.generateVizingPadProgram("devnet");

  // // check vizing program balance
  // const balance = await vizingProgram.provider.connection.getBalance(
  //   vizingProgram.programId
  // );
  // console.log("vizingPad balance: ", balance);

  const tpg = vizingUtils.addressToNumberArray(
    // "0x000000000000000000000000c3C7A782dda00a8E61Cb9Ba0ea8680bb3f3B9d10"
    "0x227d76AB1cEa2eDFc9A62833aF1743259c1f055f"
  );

  const message = {
    mode: 4,
    targetProgram: tpg,
    executeGasLimit: new anchor.BN(200000),
    maxFeePerGas: new anchor.BN(35),
    signature: Buffer.from("TEST_CODE"),
  };

  const additionParams = {
    mode: 0,
    signature: Buffer.alloc(0),
  };

  const launchParams = {
    erliestArrivalTimestamp: new anchor.BN(0),
    latestArrivalTimestamp: new anchor.BN(0),
    relayer: Buffer.from([
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0,
    ]),
    sender: vizingProgram.provider.publicKey,
    value: new anchor.BN(0),
    destChainid: new anchor.BN(28516),
    additionParams: additionParams,
    message: message,
  };

  console.log(devnetConfig);

  console.log("launchParams: ", launchParams);

  console.log("run");

  const tx = await vizingProgram.methods
    .launch(launchParams)
    .accounts({
      vizingAppFeePayer: vizingProgram.provider.publicKey!,
      vizingAppMessageAuthority: vizingProgram.provider.publicKey!,
      vizingPadConfig: vizingUtils.generatePublicKeyFromString(
        devnetConfig.vizingPadConfig
      ),
      vizingPadFeeCollector: vizingUtils.generatePublicKeyFromString(
        devnetConfig.vizingPadFeeCollector
      ),
      vizingGasSystem: vizingUtils.generatePublicKeyFromString(
        devnetConfig.vizingGasSystem
      ),
    })
    .rpc();

  console.log(`launch: ${tx}`);
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
