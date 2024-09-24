import * as anchor from "@coral-xyz/anchor";
import { VizingCore } from "../target/types/vizing_core";
import * as vizingUtils from "../migrations/vizing.utils";
import * as vizingInit from "../migrations/initial.vizingPad";

let deployer: anchor.web3.Keypair;
let feeCollector: anchor.web3.PublicKey = new anchor.web3.PublicKey(
  "feeCFx1bo4Z6a7fRvkb9xAu3eniyp6q1JF5Mwwm6TRE"
);
let engineAdmin: anchor.web3.PublicKey = new anchor.web3.PublicKey(
  "pisxReuiFWqib2JZno8MUtM6FyNe46er3s4YTHTzJLP"
);
let stationAdmin: anchor.web3.PublicKey = new anchor.web3.PublicKey(
  "pisxReuiFWqib2JZno8MUtM6FyNe46er3s4YTHTzJLP"
);
let gasPoolAdmin: anchor.web3.PublicKey = new anchor.web3.PublicKey(
  "pisxReuiFWqib2JZno8MUtM6FyNe46er3s4YTHTzJLP"
);
let swapManager: anchor.web3.PublicKey = new anchor.web3.PublicKey(
  "reLKZaXp8h1CnydbtFo4H1CSriTMvfTDa1KifqhdRjR"
);
let trustedRelayers: anchor.web3.PublicKey[] = [
  new anchor.web3.PublicKey("pisxReuiFWqib2JZno8MUtM6FyNe46er3s4YTHTzJLP"),
  new anchor.web3.PublicKey("reLKZaXp8h1CnydbtFo4H1CSriTMvfTDa1KifqhdRjR "),
  new anchor.web3.PublicKey("reLDpKcA1GX5HmZgE2XUJfRX7JTwy9fM4bSE1AMmx84  "),
  new anchor.web3.PublicKey("reLYRvr7RCrvZwmtMHZMyXZZkC8LUZ9rJ3wrECL13Ly "),
];
let registeredValidator: anchor.web3.PublicKey = new anchor.web3.PublicKey(
  "pisxReuiFWqib2JZno8MUtM6FyNe46er3s4YTHTzJLP"
);

export async function main() {
  console.log("### Deploy start");

  // const provider = pg.program.provider;
  // const programId = pg.program.programId;
  // const bobProgram = new Program(IDL, programId, provider);

  const vizingCoreProgramId = "vizngM8xTgmP15xuxpUZHbdec3LBG7bnTe9j1BtaqsE";
  const vizingAppMockProgramId = "2xiuj4ozxygvkmC1WKJTGZyJXSD8dtbFxWkuJiMLzrTg";

  let provider;
  let programId;
  let vizingProgram = new anchor.Program(IDL, programId, provider);

  let vizingAppMockProgram = new anchor.Program(
    IDL,
    vizingAppMockProgramId,
    provider
  );

  const initGasSystemParams: vizingUtils.initializeVizingGasSystemParams = {
    chainId: new anchor.BN(28516),
    basePrice: new anchor.BN(33),
    molecular: new anchor.BN(0),
    denominator: new anchor.BN(10),
    molecularDecimal: 1,
    denominatorDecimal: 1,
    globalBasePrice: new anchor.BN(33),
    defaultGasLimit: new anchor.BN(200000),
    amountInThreshold: new anchor.BN(anchor.web3.LAMPORTS_PER_SOL * 100),
    globalMolecular: new anchor.BN(0),
    globalDenominator: new anchor.BN(10),
  };

  {
    const initRet = await vizingInit.inititalizeVizingPad(
      vizingProgram,
      provider.wallet.publicKey,
      feeCollector,
      engineAdmin,
      stationAdmin,
      gasPoolAdmin,
      swapManager,
      trustedRelayers,
      registeredValidator,
      initGasSystemParams
    );

    console.log("initRet:", initRet);

    const [_resultDataAccount, resultDataBump] =
      vizingUtils.generatePdaForResultData(vizingAppMockProgram.programId);

    const resultDataAccount = _resultDataAccount;

    console.log(
      `resultDataAccount: ${resultDataAccount.toBase58()}, bump: ${resultDataBump}`
    );

    const initVizingApp = await vizingInit.initializeVizingApp(
      vizingAppMockProgram,
      provider.wallet.publicKey
    );

    console.log("initVizingApp:", initVizingApp);

    const initRegAppRet = await vizingInit.inititalizeRegisterVizingApp(
      vizingProgram,
      provider.wallet.publicKey,
      vizingAppMockProgram.programId,
      [resultDataAccount]
    );

    console.log("initRegAppRet:", initRegAppRet);
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
