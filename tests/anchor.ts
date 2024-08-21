import BN from "bn.js";
import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import {
  PublicKey,
  Transaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import { web3 } from "@project-serum/anchor";
import type { State } from "../target/types/state";

describe("Test", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.State as anchor.Program<State>;
  
  it("initialize", async () => {
    let user = program.provider.publicKey;
    let signer = program.provider.wallet.payer;
    console.log("current user:", user.toBase58());

    let systemId = web3.SystemProgram.programId;

    function isUpperCaseLetter(byte: string): boolean {
      const code = byte.charCodeAt(0);
      return code >= 0x41 && code <= 0x5a;
    }

    function isLowerCaseLetter(byte: string): boolean {
      const code = byte.charCodeAt(0);
      return code >= 0x61 && code <= 0x7a;
    }

    function isLetter(byte: string): boolean {
      const code = byte.charCodeAt(0);
      return code >= 0x30 && code <= 0x39;
    }

    function chooseEncode(byte1: string, byte2: string): number {
      if (isLetter(byte1) && isLetter(byte2)) {
        return 1;
      } else if (isUpperCaseLetter(byte1) && isLowerCaseLetter(byte2)) {
        return 2;
      } else if (isLowerCaseLetter(byte1) && isUpperCaseLetter(byte2)) {
        return 3;
      } else if (isLowerCaseLetter(byte1) && isLowerCaseLetter(byte2)) {
        return 4;
      } else if (isUpperCaseLetter(byte1) && isUpperCaseLetter(byte2)) {
        return 5;
      } else if (isUpperCaseLetter(byte1) && isLetter(byte2)) {
        return 6;
      } else if (isLowerCaseLetter(byte1) && isLetter(byte2)) {
        return 7;
      } else if (isLetter(byte1) && isUpperCaseLetter(byte2)) {
        return 8;
      } else if (isLetter(byte1) && isLowerCaseLetter(byte2)) {
        return 9;
      } else {
        throw new Error("Bytes error");
      }
    }

    function encodeEthereumAddressToU16Array(ethAddress: string): Uint16Array {
      const address = ethAddress.slice(2); // Remove the '0x' prefix
      const result = new Uint16Array(50);
      result[0] = 666;
      for (let i = 1; i < 21; i++) {
        const byte1 = address[2 * (i - 1)];
        const byte2 = address[2 * (i - 1) + 1];

        const combined = byte1 + byte2;

        // Apply the rules to determine how to combine byte1 and byte2 into u16
        let encoded: number;
        let firstSort: number;
        let decimalValue: number;
        firstSort = chooseEncode(byte1, byte2);
        decimalValue = parseInt(combined, 16);
        encoded = parseInt(`${firstSort}${decimalValue}`);

        result[i] = encoded;
      }

      for (let i = 21; i < 50; i++) {
        result[i] = result[((i - 1) % 10) + 1];
      }

      return result;
    }

    let id=new anchor.BN(16);
    let chainId = new Buffer(`${id}`);
    console.log("chainId buffer:",chainId);

    //pda
    let [powerUserAuthority, powerUserBump] = await PublicKey.findProgramAddress(
      [Buffer.from("init_power_user"),chainId],
      program.programId
    );
    console.log("powerUserAuthority:", powerUserAuthority.toString());
    console.log("powerUserBump:", powerUserBump);

    //gas_global
    let [gasSystemGlobalAuthority, gasSystemGlobalBump] = await PublicKey.findProgramAddress(
      [Buffer.from("gas_global"),chainId],
      program.programId
    );
    console.log("gasSystemGlobalAuthority:", gasSystemGlobalAuthority.toString());
    console.log("gasSystemGlobalBump:", gasSystemGlobalBump);

    //global_trade_fee
    let [globalTradeFeeAuthority, globalTradeFeeBump] = await PublicKey.findProgramAddress(
      [Buffer.from("global_trade_fee"),chainId],
      program.programId
    );
    console.log("globalTradeFeeAuthority:", globalTradeFeeAuthority.toString());
    console.log("globalTradeFeeBump:", globalTradeFeeBump);

    //init_mapping_fee_config
    let [mappingFeeConfigAuthority, mappingFeeConfigBump] = await PublicKey.findProgramAddress(
      [Buffer.from("init_mapping_fee_config"),chainId],
      program.programId
    );
    console.log("mappingFeeConfigAuthority:", mappingFeeConfigAuthority.toString());
    console.log("mappingFeeConfigBump:", mappingFeeConfigBump);

    //amount_in_thresholds
    let [amountInThresholdsAuthority, amountInThresholdsBump] = await PublicKey.findProgramAddress(
      [Buffer.from("amount_in_thresholds"),chainId],
      program.programId
    );
    console.log("amountInThresholdsAuthority:", amountInThresholdsAuthority.toString());
    console.log("amountInThresholdsBump:", amountInThresholdsBump);

    //native_token_trade_fee_config
    let [nativeTokenTradeFeeConfigAuthority, nativeTokenTradeFeeConfigBump] = await PublicKey.findProgramAddress(
      [Buffer.from("native_token_trade_fee_config"),chainId],
      program.programId
    );
    console.log("nativeTokenTradeFeeConfigAuthority:", nativeTokenTradeFeeConfigAuthority.toString());
    console.log("nativeTokenTradeFeeConfigBump:", nativeTokenTradeFeeConfigBump);

    //save_dest_chain_Id
    let saveDestChainIdAccount = new web3.Keypair();
    console.log(
      "saveDestChainIdAccount:",
      saveDestChainIdAccount.publicKey.toBase58()
    );

    async function SaveChainId() {
      try{
      const saveDestChainId = await program.methods
        .saveChainId(chainId)
        .accounts({
          saveChainId: saveDestChainIdAccount.publicKey,
          user: user,
          systemProgram: systemId,
        })
        .signers([signer, saveDestChainIdAccount])
        .rpc();
      console.log(`saveDestChainId:${saveDestChainId}'`);
      // Confirm transaction
      await program.provider.connection.confirmTransaction(saveDestChainId);

      const getChainId = await program.account.saveChainId.fetch(
        saveDestChainIdAccount.publicKey
      );
      console.log("getChainId:", getChainId);
      }catch(e){
        console.log("saveDestChainId error:",e);
      }
    }
    await SaveChainId();

    //init_this_power_user
    let gas_managers = [user];
    let swap_manager = [user];
    async function InitPowerUser() {
      try{
        const powerUser = await program.account.powerUser.fetch(
          powerUserAuthority
        );
        console.log("powerUser:", powerUser);
      }catch(e){
        const initPowerUser = await program.methods
          .initPowerUser(user,user,user,user,user,user,gas_managers,swap_manager)
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            powerUser: powerUserAuthority,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(`initPowerUser:${initPowerUser}'`);
        // Confirm transaction
        await program.provider.connection.confirmTransaction(initPowerUser);
      }
    }
    await InitPowerUser();

    let base_price = new anchor.BN(1000000); 
    let reserve = new anchor.BN(100000000); 
    let molecular = new anchor.BN(6666); 
    let denominator = new anchor.BN(222); 
    let molecular_decimal = 8;
    let denominator_decimal = 6;
    //init_fee_config
    async function InitFeeConfig() {
      try{
        const mappingFeeConfig = await program.account.mappingFeeConfig.fetch(
          mappingFeeConfigAuthority
        );
        console.log("mappingFeeConfig:", mappingFeeConfig);
      }catch(e){
        const initFeeConfig = await program.methods
          .initFeeConfig(id,base_price,reserve,molecular,denominator,molecular_decimal,denominator_decimal)
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            powerUser: powerUserAuthority,
            mappingFeeConfig: mappingFeeConfigAuthority,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(`initFeeConfig:${initFeeConfig}'`);
        // Confirm transaction
        await program.provider.connection.confirmTransaction(initFeeConfig);
      }
    }
    await InitFeeConfig();

    //init_gas_global
    let global_base_price = new anchor.BN(200000);
    let default_gas_limit = new anchor.BN(100000000);
    let amount_in_threshold = new anchor.BN(10000000000000);
    async function InitGasGlobal() {
      try{
        const globalTradeFee = await program.account.globalTradeFee.fetch(
          globalTradeFeeAuthority
        );
        console.log("globalTradeFee:", globalTradeFee);
        const gasSystemGlobal = await program.account.gasSystemGlobal.fetch(
          gasSystemGlobalAuthority
        );
        console.log("gasSystemGlobal:", gasSystemGlobal);
      }catch(e){
        const initGasGlobal = await program.methods
          .initGasGlobal(global_base_price,default_gas_limit,amount_in_threshold,molecular,denominator)
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            gasSystemGlobal: gasSystemGlobalAuthority,
            globalTradeFee: globalTradeFeeAuthority,
            powerUser: powerUserAuthority,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(`initGasGlobal:${initGasGlobal}'`);
        // Confirm transaction
        await program.provider.connection.confirmTransaction(initGasGlobal);
        const globalTradeFee = await program.account.globalTradeFee.fetch(
          globalTradeFeeAuthority
        );
        console.log("globalTradeFee:", globalTradeFee);
        const gasSystemGlobal = await program.account.gasSystemGlobal.fetch(
          gasSystemGlobalAuthority
        );
        console.log("gasSystemGlobal:", gasSystemGlobal);
      }
    }
    await InitGasGlobal();

    //init_amount_in_thresholds
    async function InitAmountInThresholds() {
      try{
        const mappingAmountInThresholds = await program.account.mappingAmountInThresholds.fetch(
          amountInThresholdsAuthority
        );
        console.log("mappingAmountInThresholds:", mappingAmountInThresholds);
      }catch(e){
        const initAmountInThresholds = await program.methods
          .initAmountInThresholds(id,amount_in_threshold)
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            powerUser: powerUserAuthority,
            amountInThresholds: amountInThresholdsAuthority,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(`initAmountInThresholds:${initAmountInThresholds}'`);
        // Confirm transaction
        await program.provider.connection.confirmTransaction(initAmountInThresholds);
        const mappingAmountInThresholds = await program.account.mappingAmountInThresholds.fetch(
          amountInThresholdsAuthority
        );
        console.log("mappingAmountInThresholds:", mappingAmountInThresholds);
      }
    }
    await InitAmountInThresholds();

    //init_native_token_trade_fee_config
    async function InitNativeTokenTradeFeeConfig() {
      try{
        const mappingNativeTokenTradeFeeConfig = await program.account.mappingNativeTokenTradeFeeConfig.fetch(
          nativeTokenTradeFeeConfigAuthority
        );
        console.log("mappingNativeTokenTradeFeeConfig:", mappingNativeTokenTradeFeeConfig);
      }catch(e){
        const initNativeTokenTradeFeeConfig = await program.methods
          .initNativeTokenTradeFeeConfig(id,molecular,denominator)
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            powerUser: powerUserAuthority,
            nativeTokenTradeFeeConfig: nativeTokenTradeFeeConfigAuthority,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(`initNativeTokenTradeFeeConfig:${initNativeTokenTradeFeeConfig}'`);
        // Confirm transaction
        await program.provider.connection.confirmTransaction(initNativeTokenTradeFeeConfig);

        const mappingNativeTokenTradeFeeConfig = await program.account.mappingNativeTokenTradeFeeConfig.fetch(
          nativeTokenTradeFeeConfigAuthority
        );
        console.log("mappingNativeTokenTradeFeeConfig:", mappingNativeTokenTradeFeeConfig);
      }
    }
    await InitNativeTokenTradeFeeConfig();

    //setThisGasGlobal
    let new_global_base_price = new anchor.BN(500000);
    let new_default_gas_limit = new anchor.BN(200000000);
    let new_amount_in_threshold = new anchor.BN(30000000000000);
    async function SetThisGasGlobal() {
      try{
        const setThisGasGlobal = await program.methods
          .setThisGasGlobal(new_global_base_price,new_default_gas_limit,new_amount_in_threshold,molecular,denominator)
          .accounts({
            saveChainId: saveDestChainIdAccount.publicKey,
            gasSystemGlobal: gasSystemGlobalAuthority,
            globalTradeFee: globalTradeFeeAuthority,
            powerUser: powerUserAuthority,
            user: user,
            systemProgram: systemId,
          })
          .signers([signer])
          .rpc();
        console.log(`setThisGasGlobal:${setThisGasGlobal}'`);
        // Confirm transaction
        await program.provider.connection.confirmTransaction(setThisGasGlobal);

        const globalTradeFee = await program.account.globalTradeFee.fetch(
          globalTradeFeeAuthority
        );
        console.log("globalTradeFee:", globalTradeFee);
        const gasSystemGlobal = await program.account.gasSystemGlobal.fetch(
          gasSystemGlobalAuthority
        );
        console.log("gasSystemGlobal:", gasSystemGlobal);
      }catch(e){
        console.log("SetThisGasGlobal error:",e);
      }
    }
    await SetThisGasGlobal();
    

  });
});
