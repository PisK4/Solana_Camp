import * as anchor from "@coral-xyz/anchor";
import * as vizingUtils from "./vizing.utils";
import * as vizingInit from "./initial.vizingPad";

export async function main() {
  console.log("### init start");
  const vizingProgram = vizingUtils.generateVizingPadProgram("devnet");

  console.log("run with vizingProgram: ", vizingProgram.programId.toBase58());

  const vizingAppMockProgram = vizingUtils.generateVizingAppMockProgram(
    "devnet"
  ) as any;

  console.log(
    "run with vizingAppMockProgram: ",
    vizingAppMockProgram.programId.toBase58()
  );

  const balance = await vizingAppMockProgram.provider.connection.getBalance(
    vizingAppMockProgram.programId
  );
  console.log("vizingAppMockProgram balance: ", balance);

  const ret1 = await vizingInit.initializeVizingAppMock(
    vizingAppMockProgram,
    vizingAppMockProgram.provider.publicKey
  );

  const ret2 = await vizingInit.initializeVizingApp(
    vizingAppMockProgram,
    vizingAppMockProgram.provider.publicKey
  );

  // const resultDataAccount = vizingUtils.generatePublicKeyFromString(
  //   "AN6Ujf4gZudtu7EGFUsg5xHswwsR2Mke3kenoEQQk1ya"
  // );

  const resultDataAccount = ret1.resultDataAccount;

  await vizingInit.inititalizeRegisterVizingApp(
    vizingProgram,
    vizingProgram.provider.publicKey,
    vizingAppMockProgram.programId,
    [resultDataAccount]
  );
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
