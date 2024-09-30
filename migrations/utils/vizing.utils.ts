import * as anchor from "@coral-xyz/anchor";
import fs from "fs";
import vizingCoreIDL from "../../target/idl/vizing_core.json";
import vizingAppMockIDL from "../../target/idl/vizing_app_mock.json";

// **** Vizing Pad Configs ***
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

export interface initializeVizingPadParams {
  owner: anchor.web3.PublicKey;
  feeCollector: anchor.web3.PublicKey;
  engineAdmin: anchor.web3.PublicKey;
  gasPoolAdmin: anchor.web3.PublicKey;
  swapManager: anchor.web3.PublicKey;
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

export interface initializeVizingGasSystemParams {
  groupId: anchor.BN;
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

export function generatePdaForVizingGasSystem(
  vizingPadProgramId: anchor.web3.PublicKey,
  vizingPadConfig: anchor.web3.PublicKey,
  groupId: anchor.BN
): [anchor.web3.PublicKey, number] {
  return pdaFromSeeds(
    [
      vizingGasSystemSeed,
      vizingPadConfig.toBuffer(),
      groupId.toBuffer("be", 8),
    ],
    vizingPadProgramId
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
      vizingGasSystem: accounts.vizingGasSystem,
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

// **** utils ****

export function loadKeypairFromFile(filename: string): anchor.web3.Keypair {
  const secret = JSON.parse(fs.readFileSync(filename).toString()) as number[];
  const secretKey = Uint8Array.from(secret);
  return anchor.web3.Keypair.fromSecretKey(secretKey);
}

export function loadProgramKeypairFromFile(
  program: string
): anchor.web3.Keypair {
  let filename: string = "target/deploy/";
  switch (program) {
    case "vizingCore":
      filename += "vizing_core-keypair.json";
      break;
    case "vizingAppMock":
      filename += "vizing_app_mock-keypair.json";
      break;
    default:
      throw new Error(`Program not supported: ${program}`);
  }
  return loadKeypairFromFile(filename);
}

export function padStringTo32Bytes(str: string): Buffer {
  const buffer = Buffer.alloc(32);
  buffer.write(str.replace("0x", ""), "hex");
  return buffer;
}

export function padEthereumAddressToBuffer(address: string): Buffer {
  const cleanAddress = address.startsWith("0x") ? address.slice(2) : address;
  const buffer = Buffer.alloc(32);
  buffer.write(
    cleanAddress,
    32 - cleanAddress.length / 2,
    cleanAddress.length / 2,
    "hex"
  );
  return buffer;
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

/**
 * @param address contract address
 * @returns number[]
 * 0x3fC91A3afd70395Cd496C647d5a6CC9D4B2b7F11 -> [
    63, 201,  26,  58, 253, 112,
    57,  92, 212, 150, 198,  71,
    213, 166, 204, 157,  75,  43,
    127,  17
  ]
 */
export function addressToNumberArray(address: string): number[] {
  return Array.from(Buffer.from(address.replace("0x", ""), "hex"));
}

// **** vizing app ****
interface vizingAppRegisterParams {
  solPdaReceiver: anchor.web3.PublicKey;
  vizingAppAccounts: anchor.web3.PublicKey[];
  vizingAppProgramId: anchor.web3.PublicKey;
}

export function generatePdaForVizingAppConfig(
  vizingPadProgramId: anchor.web3.PublicKey,
  vizingAppProgramId: anchor.web3.PublicKey
): [anchor.web3.PublicKey, number] {
  return pdaFromSeeds(
    [vizingAppConfigSeed, vizingAppProgramId.toBuffer()],
    vizingPadProgramId
  );
}

export function generatePdaForVizingAppSolReceiver(
  vizingAppProgramId: anchor.web3.PublicKey
): [anchor.web3.PublicKey, number] {
  return pdaFromSeeds([vizingAppSolReceiverSeed], vizingAppProgramId);
}

export async function vizingAppRegister(
  vizingProgram: anchor.Program,
  params: vizingAppRegisterParams,
  admin: anchor.web3.PublicKey,
  vizingAppConfig: anchor.web3.PublicKey
): Promise<anchor.web3.TransactionSignature> {
  return vizingProgram.methods
    .registerVizingApp(params)
    .accounts({
      admin: admin,
      vizingAppConfigs: vizingAppConfig,
    })
    .rpc();
}

// **** mock vizing app ****
export const resultDataSeed = Buffer.from("result_data_seed");

export function generatePdaForResultData(
  vizingMockAppprogramId: anchor.web3.PublicKey
): [anchor.web3.PublicKey, number] {
  return pdaFromSeeds([resultDataSeed], vizingMockAppprogramId);
}

interface launchAccounts {
  user: anchor.web3.PublicKey;
  vizingAppMessageAuthority: anchor.web3.PublicKey;
  vizingPadConfig: anchor.web3.PublicKey;
  vizingPadFeeCollector: anchor.web3.PublicKey;
  vizingPadProgram: anchor.web3.PublicKey;
  vizingGasSystem: anchor.web3.PublicKey;
  currentRecordMessage: anchor.web3.PublicKey;
}

export async function launchFromVizingApp(
  vizingAppMockProgram: anchor.Program,
  targetProgram: number[],
  meta: Buffer,
  accounts: launchAccounts
): Promise<anchor.web3.TransactionSignature> {
  const expectedFee = new anchor.BN(7000000);
  return vizingAppMockProgram.methods
    .launchVizing(targetProgram, meta, expectedFee)
    .accounts({
      user: accounts.user,
      vizingAppMessageAuthority: accounts.vizingAppMessageAuthority,
      vizingPadConfig: accounts.vizingPadConfig,
      vizingPadFeeCollector: accounts.vizingPadFeeCollector,
      vizingPadProgram: accounts.vizingPadProgram,
      vizingGasSystem: accounts.vizingGasSystem,
      currentRecordMessage: accounts.currentRecordMessage,
    })
    .rpc();
}

export function generateVizingPadProgram(
  network: string = "devnet"
): anchor.Program {
  let programId: string;
  switch (network) {
    case "devnet":
      programId = "vizngM8xTgmP15xuxpUZHbdec3LBG7bnTe9j1BtaqsE";
      break;
    default:
      throw new Error(`Network not supported: ${network}`);
  }

  const provider = getProvider(network);
  return new anchor.Program(vizingCoreIDL as anchor.Idl, provider);
}

export function generateVizingAppMockProgram(
  network: string = "devnet"
): anchor.Program {
  let programId: string;
  switch (network) {
    case "devnet":
      programId = "mokB6FzEZx6vPVmasd19CyDDuqZ98auke1Bk59hmzVE";
      break;
    default:
      throw new Error(`Network not supported: ${network}`);
  }

  const provider = getProvider(network);
  return new anchor.Program(vizingAppMockIDL as anchor.Idl, provider);
}

export function getProvider(network: string = "devnet"): anchor.Provider {
  let url: string;
  switch (network) {
    case "devnet":
      url =
        "https://solana-devnet.g.alchemy.com/v2/-m2gJ2Fiv4w403IMR27nGoHUyonc0azl";
      break;
    case "mainnet":
      url =
        "https://solana-mainnet.g.alchemy.com/v2/-m2gJ2Fiv4w403IMR27nGoHUyonc0azl";
      break;
    case "local":
      url = "http://127.0.0.1:8899";
      break;
    default:
      throw new Error(`Network not supported: ${network}`);
  }

  const connection = new anchor.web3.Connection(url, "confirmed");
  const provider = new anchor.AnchorProvider(
    connection,
    anchor.AnchorProvider.local().wallet,
    {
      commitment: "confirmed",
    }
  );

  return provider;
}

export function generatePublicKeyFromString(
  address: string
): anchor.web3.PublicKey {
  return new anchor.web3.PublicKey(address);
}

export const formatReturnInfo = (ret: any) => {
  return Object.fromEntries(
    Object.entries(ret).map(([key, value]) => {
      if (value instanceof anchor.web3.PublicKey) {
        return [key, value.toBase58()];
      }
      return [key, value];
    })
  );
};
