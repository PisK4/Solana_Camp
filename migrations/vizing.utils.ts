import * as anchor from "@coral-xyz/anchor";

export const VizingPadConfigsSeed = Buffer.from("Vizing_Pad_Settings_Seed");
export const vizingAuthoritySeed = Buffer.from("Vizing_Authority_Seed");
export const vizingAppConfigSeed = Buffer.from("Vizing_App_Config_Seed");
export const vizingAppSolReceiverSeed = Buffer.from(
  "Vizing_App_Sol_Receiver_Seed"
);
export const vizingFeeRouterSeed = Buffer.from("Vizing_Fee_Router_Seed");
export const vizingMessageAuthoritySeed = Buffer.from(
  "Vizing_Message_Authority_Seed"
);
export const vizingGasSystemSeed = Buffer.from("init_mapping_fee_config");

interface initializeVizingPadParams {
  owner: anchor.web3.PublicKey;
  feeCollector: anchor.web3.PublicKey;
  engineAdmin: anchor.web3.PublicKey;
  gasPoolAdmin: anchor.web3.PublicKey;
  stationAdmin: anchor.web3.PublicKey;
  trustedRelayers: anchor.web3.PublicKey[];
  registeredValidator: anchor.web3.PublicKey;
  relayers: anchor.web3.PublicKey[];
  isPaused: boolean;
}

interface initializeVizingPadAccounts {
  vizingPadConfig: anchor.web3.PublicKey;
  vizingPadAuthority: anchor.web3.PublicKey;
  payer: anchor.web3.PublicKey;
}

interface initializeVizingGasSystemParams {
  chainId: anchor.BN;
  basePrice: anchor.BN;
  molecular: anchor.BN;
  denominator: anchor.BN;
  molecularDecimal: number;
  denominatorDecimal: number;
  globalBasePrice: anchor.BN;
  defaultGasLimit: anchor.BN;
  amountInThreshold: anchor.BN;
  globalMolecular: anchor.BN;
  globalDenominator: anchor.BN;
}

interface initializeVizingGasSystemAccounts {
  vizingPadConfig: anchor.web3.PublicKey;
  vizingGasSystem: anchor.web3.PublicKey;
  payer: anchor.web3.PublicKey;
}

export function pdaFromSeeds(
  seeds: Buffer[],
  programId: anchor.web3.PublicKey
): [anchor.web3.PublicKey, number] {
  const [pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    seeds,
    programId
  );
  return [pubkey, bump];
}

export function generatePdaForVizingPadConfig(
  programId: anchor.web3.PublicKey
): [anchor.web3.PublicKey, number] {
  return pdaFromSeeds([VizingPadConfigsSeed], programId);
}

export function generatePdaForVizingAuthority(
  programId: anchor.web3.PublicKey,
  vizingPadConfig: anchor.web3.PublicKey
): [anchor.web3.PublicKey, number] {
  return pdaFromSeeds(
    [vizingAuthoritySeed, vizingPadConfig.toBuffer()],
    programId
  );
}

export function generatePdaForRecordMessage(
  programId: anchor.web3.PublicKey
): [anchor.web3.PublicKey, number] {
  return pdaFromSeeds([Buffer.from("init_current_record_message")], programId);
}

export async function initializeVizingPad(
  program: anchor.Program,
  params: initializeVizingPadParams,
  accounts: initializeVizingPadAccounts
): Promise<anchor.web3.TransactionSignature> {
  return program.methods
    .initializeVizingPad(params)
    .accounts({
      vizingPadConfig: accounts.vizingPadConfig,
      vizingPadAuthority: accounts.vizingPadAuthority,
      payer: accounts.payer,
    })
    .rpc();
}

export async function initializeVizingGasSystem(
  program: anchor.Program,
  params: initializeVizingGasSystemParams,
  accounts: initializeVizingGasSystemAccounts
): Promise<anchor.web3.TransactionSignature> {
  return program.methods
    .initializeGasSystem(params)
    .accounts({
      vizingPadConfig: accounts.vizingPadConfig,
      mappingFeeConfig: accounts.vizingGasSystem,
      payer: accounts.payer,
    })
    .rpc();
}

export async function initializeRecordMessage(
  program: anchor.Program,
  currentRecordMessage: anchor.web3.PublicKey,
  user: anchor.web3.PublicKey
): Promise<anchor.web3.TransactionSignature> {
  return program.methods
    .initRecordMessage()
    .accounts({
      currentRecordMessage: currentRecordMessage,
      user: user,
    })
    .rpc();
}
