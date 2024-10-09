import * as anchor from "@coral-xyz/anchor";
import * as vizingUtils from "../utils/vizing.utils";
import devnetConfig from "../deployment.devnet.json";

export async function main() {
  console.log("### Launch start");
  const vizingAppMockProgram = vizingUtils.generateVizingAppMockProgram(
    "devnet"
  ) as any;

  const networkConfig = devnetConfig;

  // check vizing program balance
  const balance = await vizingAppMockProgram.provider.connection.getBalance(
    vizingAppMockProgram.programId
  );
  console.log("vizingPad balance: ", balance);

  const targetProgram = vizingUtils.addressToNumberArray(
    // "0x000000000000000000000000c3C7A782dda00a8E61Cb9Ba0ea8680bb3f3B9d10"
    "0x000000000000000000000000922BEA61bc185c4Ef7cF24bd1B4123a50676d6E8"
  );

  const meta = Buffer.concat([
    Buffer.from([0, 0, 0, 0, 0, 0, 0, 1]),
    Buffer.from([0, 0, 0, 0, 0, 0, 0, 9]),
  ]);

  console.log("run");

  const [vizingAppMessageAuthority, bump] = vizingUtils.pdaFromSeeds(
    [Buffer.from("Vizing_Message_Authority_Seed")],
    vizingAppMockProgram.programId
  );

  // // fetch account
  // const vizingAppMessageAuthorityAccount =
  //   await vizingAppMockProgram.account.vizingAppMessageAuthority.fetch(
  //     vizingAppMessageAuthority
  //   );
  // console.log(
  //   "vizingAppMessageAuthorityAccount: ",
  //   vizingAppMessageAuthorityAccount
  // );

  console.log(
    "balance of appAuthority",
    await vizingAppMockProgram.provider.connection.getBalance(
      vizingAppMessageAuthority
    )
  );

  // return;

  console.log("targetProgram", targetProgram);

  try {
    const tx = await vizingUtils.launchFromVizingApp(
      vizingAppMockProgram,
      targetProgram,
      meta,
      {
        user: vizingAppMockProgram.provider.publicKey,
        vizingAppMessageAuthority: vizingAppMessageAuthority,
        vizingPadConfig: vizingUtils.generatePublicKeyFromString(
          networkConfig.vizingPadConfig
        ),
        vizingPadFeeCollector: vizingUtils.generatePublicKeyFromString(
          networkConfig.vizingPadFeeCollector
        ),
        vizingPadProgram: vizingUtils.generatePublicKeyFromString(
          networkConfig.vizingPadProgram
        ),
        vizingGasSystem: vizingUtils.generatePublicKeyFromString(
          networkConfig.vizingGasSystem
        ),
        currentRecordMessage: vizingUtils.generatePublicKeyFromString(
          networkConfig.currentRecordMessage
        ),
      }
    );

    console.log(`launch: ${tx}`);
  } catch (error) {
    if (error.signature) {
      console.log(`signature: ${error.signature}`);
    } else {
      console.error(error);
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
