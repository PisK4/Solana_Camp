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

  // try {
  //   await vizingInit.initializeVizingAppMock(
  //     vizingAppMockProgram,
  //     vizingAppMockProgram.provider.publicKey
  //   );
  // } catch (error) {
  //   console.error(error);
  // }

  // try {
  //   await vizingInit.initializeVizingApp(
  //     vizingAppMockProgram,
  //     vizingAppMockProgram.provider.publicKey
  //   );
  // } catch (error) {
  //   console.error(error);
  // }

  const resultDataAccount = vizingUtils.generatePublicKeyFromString(
    "AN6Ujf4gZudtu7EGFUsg5xHswwsR2Mke3kenoEQQk1ya"
  );

  const solPdaReceiver = vizingUtils.generatePublicKeyFromString(
    "5jPBGc4fCev7a65135eknhRjayuBLrA6xEs7DKTqp2uw"
  );

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
