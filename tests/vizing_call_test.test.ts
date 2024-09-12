import * as anchor from "@coral-xyz/anchor";
import { Program, Coder } from "@coral-xyz/anchor";
import { expect } from "chai";
import {
    PublicKey
  } from "@solana/web3.js";
import { VizingCallTest } from "../target/types/vizing_call_test";

describe("Vizing Test", () => {
    it("Test", async () => {
        const provider = anchor.AnchorProvider.env();

        // Configure the client to use the local cluster.
        anchor.setProvider(provider);
        const vizingCallTestProgram = anchor.workspace.VizingCallTest as Program<VizingCallTest>;
        
        let PROGRAM_ID=vizingCallTestProgram.programId;

        let TARGET_ID=new PublicKey("Target Program Id");

        let user=provider.wallet.publicKey;
        console.log("user:",user.toBase58());

        const [vizingPadConfigAuthority, vizingPadConfigBump] = PublicKey.findProgramAddressSync(
            [Buffer.from("Vizing_Pad_Settings_Seed")],
            PROGRAM_ID
        );
        console.log("vizingPadConfigAuthority:",vizingPadConfigAuthority);

        //init_mapping_fee_config
        let [mappingFeeConfigAuthority, mappingFeeConfigBump] =
        await PublicKey.findProgramAddress(
            [Buffer.from("init_mapping_fee_config")],
            PROGRAM_ID
            );
        console.log(
            "mappingFeeConfigAuthority:",
            mappingFeeConfigAuthority.toString()
        );
        console.log("mappingFeeConfigBump:", mappingFeeConfigBump);

        //init_current_record_message
        let [recordMessageAuthority, recordMessageBump] =
        await PublicKey.findProgramAddress(
            [Buffer.from("init_current_record_message")],
            PROGRAM_ID
        );
        console.log("recordMessageAuthority:", recordMessageAuthority.toString());
        console.log("recordMessageBump:", recordMessageBump);

        let dest_chain_id=new anchor.BN(1101);
        let recordMessageAccount=new anchor.web3.Keypair();
        console.log("recordMessageAccount:",recordMessageAccount.publicKey.toBase58());

        function ethereumAddressToU8Array(address: string): number[] {
            const cleanAddress = address.startsWith("0x")
              ? address.slice(2)
              : address;
            const bytes = new Uint8Array(32);
            for (let i = 0; i < 32; i++) {
              const byte = parseInt(cleanAddress.substr(i * 2, 2), 16);
              bytes[i] = byte;
            }
            const addressArray: number[] = Array.from(bytes);
            return addressArray;
        }

        //initialize
        async function Initialize(){
            const initializ=await vizingCallTestProgram.methods.initialize().accounts({
                recordMessage: recordMessageAccount.publicKey,
                user: user,
                systemProgram: PROGRAM_ID,
            })
            .signers([recordMessageAccount])
            .rpc();
            console.log(`getComputeTradeFee1 tx:${initializ}'`);
            // Confirm transaction
            await provider.connection.confirmTransaction(initializ);
        }
        await Initialize();

        //
        let computeTradeFee1AmountOut=new anchor.BN(10000);
        async function TestGetComputeTradeFee1(chainId,amountOut){
            const getComputeTradeFee1=await vizingCallTestProgram.methods.getComputeTradeFee1(
                chainId,
                amountOut
            ).accounts({
                mappingFeeConfig: mappingFeeConfigAuthority,
                currentRecordMessage: recordMessageAuthority,
                recordMessage: recordMessageAccount.publicKey,
                user: user,
                vizingPadProgram: TARGET_ID,
                systemProgram: PROGRAM_ID,
            })
            .signers([])
            .rpc();
            console.log(`getComputeTradeFee1 tx:${getComputeTradeFee1}'`);
            // Confirm transaction
            await provider.connection.confirmTransaction(getComputeTradeFee1);
            const testRecordAccount=await vizingCallTestProgram.account.testRecordAccount.fetch(
                recordMessageAccount.publicKey
            );
            const result=testRecordAccount.getRecordNumber.toNumber();
            console.log("result:",result);
        }
        await TestGetComputeTradeFee1(dest_chain_id,computeTradeFee1AmountOut);

        //
        let testTargetContract=Buffer.from("0xdAC17F958D2ee523a2206206994597C13D831ec7");
        async function TestGetComputeTradeFee2(targetContract,chainId,amountOut){
            const getComputeTradeFee2=await vizingCallTestProgram.methods.getComputeTradeFee2(
                targetContract,
                chainId,
                amountOut
            ).accounts({
                mappingFeeConfig: mappingFeeConfigAuthority,
                currentRecordMessage: recordMessageAuthority,
                recordMessage: recordMessageAccount.publicKey,
                user: user,
                vizingPadProgram: TARGET_ID,
                systemProgram: PROGRAM_ID,
            })
            .signers([])
            .rpc();
            console.log(`getComputeTradeFee2 tx:${getComputeTradeFee2}'`);
            // Confirm transaction
            await provider.connection.confirmTransaction(getComputeTradeFee2);
            const testRecordAccount=await vizingCallTestProgram.account.testRecordAccount.fetch(
                recordMessageAccount.publicKey
            );
            const result=testRecordAccount.getRecordNumber.toNumber();
            console.log("result:",result);
        }
        await TestGetComputeTradeFee2(testTargetContract,dest_chain_id,computeTradeFee1AmountOut);

        //get_estimate_price1
        async function TestGetEstimatePrice1(targetContract,chainId){
            const getEstimatePrice1=await vizingCallTestProgram.methods.getEstimatePrice1(
                targetContract,
                chainId
            ).accounts({
                mappingFeeConfig: mappingFeeConfigAuthority,
                currentRecordMessage: recordMessageAuthority,
                recordMessage: recordMessageAccount.publicKey,
                user: user,
                vizingPadProgram: TARGET_ID,
                systemProgram: PROGRAM_ID,
            })
            .signers([])
            .rpc();
            console.log(`getEstimatePrice1 tx:${getEstimatePrice1}'`);
            // Confirm transaction
            await provider.connection.confirmTransaction(getEstimatePrice1);
            const testRecordAccount=await vizingCallTestProgram.account.testRecordAccount.fetch(
                recordMessageAccount.publicKey
            );
            const result=testRecordAccount.getRecordNumber.toNumber();
            console.log("result:",result);
        }
        await TestGetEstimatePrice1(testTargetContract,dest_chain_id);

        //get_estimate_price2
        async function TestGetEstimatePrice2(chainId){
            const getEstimatePrice2=await vizingCallTestProgram.methods.getEstimatePrice2(
                chainId
            ).accounts({
                mappingFeeConfig: mappingFeeConfigAuthority,
                currentRecordMessage: recordMessageAuthority,
                recordMessage: recordMessageAccount.publicKey,
                user: user,
                vizingPadProgram: TARGET_ID,
                systemProgram: PROGRAM_ID,
            })
            .signers([])
            .rpc();
            console.log(`getEstimatePrice2 tx:${getEstimatePrice2}'`);
            // Confirm transaction
            await provider.connection.confirmTransaction(getEstimatePrice2);
            const testRecordAccount=await vizingCallTestProgram.account.testRecordAccount.fetch(
                recordMessageAccount.publicKey
            );
            const result=testRecordAccount.getRecordNumber.toNumber();
            console.log("result:",result);
        }
        await TestGetEstimatePrice2(dest_chain_id);

        let estimateGasAmountOut=new anchor.BN(100000);
        const testExecuteGasLimit = new anchor.BN(10);
        let testMaxFeePerGas= new anchor.BN(10000);
        let dapp = ethereumAddressToU8Array(
            "0x1b06677de21ce8B3C8970dAd08970A04DaF99756"
        );
        const newMessage = {
            mode: 1,
            targetContract: dapp,
            executeGasLimit: testExecuteGasLimit,
            maxFeePerGas: testMaxFeePerGas,
            signature: Buffer.from("transfer from alice to bob 10 usdt"),
          };
        async function TestGetEstimateGas(amountOut,chainId,message){
            const getEstimateGas=await vizingCallTestProgram.methods.getEstimatePrice2(
                amountOut,
                chainId,
                message
            ).accounts({
                mappingFeeConfig: mappingFeeConfigAuthority,
                currentRecordMessage: recordMessageAuthority,
                recordMessage: recordMessageAccount.publicKey,
                user: user,
                vizingPadProgram: TARGET_ID,
                systemProgram: PROGRAM_ID,
            })
            .signers([])
            .rpc();
            console.log(`getEstimateGas tx:${getEstimateGas}'`);
            // Confirm transaction
            await provider.connection.confirmTransaction(getEstimateGas);
            const testRecordAccount=await vizingCallTestProgram.account.testRecordAccount.fetch(
                recordMessageAccount.publicKey
            );
            const result=testRecordAccount.getRecordNumber.toNumber();
            console.log("result:",result);
        }
        await TestGetEstimateGas(estimateGasAmountOut,dest_chain_id,newMessage);

        //
        async function TestGetEstimateTotalFee(chainId,amountOut,message){
            const getEstimateTotalFee=await vizingCallTestProgram.methods.getEstimateTotalFee(
                chainId,
                amountOut,
                message
            ).accounts({
                mappingFeeConfig: mappingFeeConfigAuthority,
                currentRecordMessage: recordMessageAuthority,
                recordMessage: recordMessageAccount.publicKey,
                user: user,
                vizingPadProgram: TARGET_ID,
                systemProgram: PROGRAM_ID,
            })
            .signers([])
            .rpc();
            console.log(`getEstimateTotalFee tx:${getEstimateTotalFee}'`);
            // Confirm transaction
            await provider.connection.confirmTransaction(getEstimateTotalFee);
            const testRecordAccount=await vizingCallTestProgram.account.testRecordAccount.fetch(
                recordMessageAccount.publicKey
            );
            const result=testRecordAccount.getRecordNumber.toNumber();
            console.log("result:",result);
        }
        await TestGetEstimateTotalFee(dest_chain_id,estimateGasAmountOut,newMessage);

        //
        let exactOutputAmountOut=new anchor.BN(10000);
        async function TestGetExactOutput(chainId,amountOut){
            const getExactOutput=await vizingCallTestProgram.methods.getExactOutput(
                chainId,
                amountOut
            ).accounts({
                mappingFeeConfig: mappingFeeConfigAuthority,
                currentRecordMessage: recordMessageAuthority,
                recordMessage: recordMessageAccount.publicKey,
                user: user,
                vizingPadProgram: TARGET_ID,
                systemProgram: PROGRAM_ID,
            })
            .signers([])
            .rpc();
            console.log(`getExactOutput tx:${getExactOutput}'`);
            // Confirm transaction
            await provider.connection.confirmTransaction(getExactOutput);
            const testRecordAccount=await vizingCallTestProgram.account.testRecordAccount.fetch(
                recordMessageAccount.publicKey
            );
            const result=testRecordAccount.getRecordNumber.toNumber();
            console.log("result:",result);
        }
        await TestGetExactOutput(dest_chain_id,exactOutputAmountOut);

        async function TestGetExactInput(chainId,amountIn){
            const getExactInput=await vizingCallTestProgram.methods.getExactInput(
                chainId,
                amountIn
            ).accounts({
                mappingFeeConfig: mappingFeeConfigAuthority,
                currentRecordMessage: recordMessageAuthority,
                recordMessage: recordMessageAccount.publicKey,
                user: user,
                vizingPadProgram: TARGET_ID,
                systemProgram: PROGRAM_ID,
            })
            .signers([])
            .rpc();
            console.log(`getExactInput tx:${getExactInput}'`);
            // Confirm transaction
            await provider.connection.confirmTransaction(getExactInput);
            const testRecordAccount=await vizingCallTestProgram.account.testRecordAccount.fetch(
                recordMessageAccount.publicKey
            );
            const result=testRecordAccount.getRecordNumber.toNumber();
            console.log("result:",result);
        }
        await TestGetExactInput(dest_chain_id,exactOutputAmountOut);

        let newAdditionParams=Buffer.from("");

        async function TestGetEstimateVizingGasFee1(value,chainId,additionParams,message){
            const getEstimateVizingGasFee1=await vizingCallTestProgram.methods.getEstimateVizingGasFee1(
                value,
                chainId,
                additionParams,
                message
            ).accounts({
                mappingFeeConfig: mappingFeeConfigAuthority,
                currentRecordMessage: recordMessageAuthority,
                recordMessage: recordMessageAccount.publicKey,
                user: user,
                vizingPadProgram: TARGET_ID,
                systemProgram: PROGRAM_ID,
            })
            .signers([])
            .rpc();
            console.log(`getEstimateVizingGasFee1 tx:${getEstimateVizingGasFee1}'`);
            // Confirm transaction
            await provider.connection.confirmTransaction(getEstimateVizingGasFee1);
            const testRecordAccount=await vizingCallTestProgram.account.testRecordAccount.fetch(
                recordMessageAccount.publicKey
            );
            const result=testRecordAccount.getRecordNumber.toNumber();
            console.log("result:",result);
        }
        await TestGetEstimateVizingGasFee1(estimateGasAmountOut,dest_chain_id,newAdditionParams,newMessage);

    });
});