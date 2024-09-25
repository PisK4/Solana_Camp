import * as anchor from "@coral-xyz/anchor";
import { VizingCore } from "../target/types/vizing_core";
import * as vizingUtils from "../migrations/vizing.utils";
import * as vizingInit from "../migrations/initial.vizingPad";

let deployer: anchor.web3.PublicKey = new anchor.web3.PublicKey(
  "pisxReuiFWqib2JZno8MUtM6FyNe46er3s4YTHTzJLP"
);
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
  console.log("### vizing init start");

  const vizingCoreProgramId = "vizngM8xTgmP15xuxpUZHbdec3LBG7bnTe9j1BtaqsE";
  const vizingAppMockProgramId = "2xiuj4ozxygvkmC1WKJTGZyJXSD8dtbFxWkuJiMLzrTg";

  let vizingProgram = vizingUtils.generateVizingPadProgram("devnet");

  if (deployer != vizingProgram.provider.publicKey) {
    throw new Error(
      "deployer keyPair wallet not match, set your keyPair wallet on Anhcor.toml file"
    );
  }

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

  const initRet = await vizingInit.inititalizeVizingPad(
    vizingProgram,
    vizingProgram.provider.publicKey,
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

  console.log("### vizing init end");
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
