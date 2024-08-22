import * as anchor from "@coral-xyz/anchor";
import { Program, Coder } from "@coral-xyz/anchor";
import { expect } from "chai";

import { VizingCore } from "../target/types/vizing_core";
import { VizingApp } from "../target/types/vizing_app";
import { VizingAppMock } from "../target/types/vizing_app_mock";
import { sha256 } from "@coral-xyz/anchor/dist/cjs/utils";

function padStringTo32Bytes(str: string): Buffer {
  const buffer = Buffer.alloc(32);
  buffer.write(str);
  return buffer;
}

describe("Vizing Test", () => {
  const provider = anchor.AnchorProvider.env();

  // Configure the client to use the local cluster.
  anchor.setProvider(provider);
  const vizingProgram = anchor.workspace.VizingCore as Program<VizingCore>;
  const vizingAppProgram = anchor.workspace.VizingApp as Program<VizingApp>;
  const vizingAppMockProgram = anchor.workspace
    .VizingAppMock as Program<VizingAppMock>;
  const vizingPadSettingsSeed = Buffer.from("Vizing_Pad_Settings_Seed");
  // const relayerSettingsSeed = Buffer.from("Relayer_Settings_Seed");
  const vizingAuthoritySeed = Buffer.from("Vizing_Authority_Seed");

  let vizingPadSettings: anchor.web3.PublicKey;
  let relayerSettings: anchor.web3.PublicKey;
  let vizingAuthority: anchor.web3.PublicKey;

  const feeReceiverKeyPair = anchor.web3.Keypair.fromSeed(
    Buffer.from(padStringTo32Bytes("fee_receiver"))
  );

  const engineAdminKeyPairs = [
    anchor.web3.Keypair.fromSeed(
      Buffer.from(padStringTo32Bytes("engine_admin_1"))
    ),
    anchor.web3.Keypair.fromSeed(
      Buffer.from(padStringTo32Bytes("engine_admin_2"))
    ),
  ];

  const stationAdminKeyPair = anchor.web3.Keypair.fromSeed(
    Buffer.from(padStringTo32Bytes("station_admim"))
  );

  const gasPoolAdminKeyPair = anchor.web3.Keypair.fromSeed(
    Buffer.from(padStringTo32Bytes("gas_pool_admin"))
  );

  const trustedRelayerKeyPairs = [
    anchor.web3.Keypair.fromSeed(
      Buffer.from(padStringTo32Bytes("trusted_relayer_1"))
    ),
    anchor.web3.Keypair.fromSeed(
      Buffer.from(padStringTo32Bytes("trusted_relayer_2"))
    ),
  ];

  const registeredValidatorKeyPairs = [
    anchor.web3.Keypair.fromSeed(
      Buffer.from(padStringTo32Bytes("registered_validator_1"))
    ),
    anchor.web3.Keypair.fromSeed(
      Buffer.from(padStringTo32Bytes("registered_validator_2"))
    ),
  ];

  it("Initializes Vizing Pad", async () => {
    console.log("start initialize");
    let vizingPadBump: number;
    let relayerBump: number;
    {
      const seed = [vizingPadSettingsSeed];
      const [vizingPad, bump] = anchor.web3.PublicKey.findProgramAddressSync(
        seed,
        vizingProgram.programId
      );

      vizingPadSettings = vizingPad;
      vizingPadBump = bump;

      console.log(`vizingPad: ${vizingPad.toBase58()}, bump: ${bump}`);
    }

    const initParams = {
      owner: provider.wallet.publicKey,
      feeReceiver: feeReceiverKeyPair.publicKey,
      engineAdmin: engineAdminKeyPairs.map((keypair) => keypair.publicKey)[0],
      gasPoolAdmin: gasPoolAdminKeyPair.publicKey,
      stationAdmin: stationAdminKeyPair.publicKey,
      trustedRelayers: trustedRelayerKeyPairs.map(
        (keypair) => keypair.publicKey
      ),
      registeredValidator: registeredValidatorKeyPairs.map(
        (keypair) => keypair.publicKey
      )[0],
      relayers: trustedRelayerKeyPairs.map((keypair) => keypair.publicKey),
      isPaused: false,
    };
    // {
    //   const seed = [relayerSettingsSeed, initParams.trustedRelayer.toBuffer()];
    //   const [relayer, bump] = anchor.web3.PublicKey.findProgramAddressSync(
    //     seed,
    //     vizingProgram.programId
    //   );

    //   relayerSettings = relayer;
    //   relayerBump = bump;

    //   console.log(`relayer: ${relayer.toBase58()}, bump: ${bump}`);
    // }

    {
      const seed = [vizingAuthoritySeed];
      const [authority, bump] = anchor.web3.PublicKey.findProgramAddressSync(
        seed,
        vizingProgram.programId
      );

      console.log(`authority: ${authority.toBase58()}, bump: ${bump}`);
      vizingAuthority = authority;
    }

    {
      const tx = await vizingProgram.methods
        .initializeVizingPad(initParams)
        .accounts({
          vizing: vizingPadSettings,
          vizingAuthority: vizingAuthority,
          payer: provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
      console.log(`initialize: ${tx}`);

      const vizingPadAccount =
        await vizingProgram.account.vizingPadSettings.fetch(vizingPadSettings);

      expect(vizingPadAccount.owner.toBase58()).to.equal(
        provider.wallet.publicKey.toBase58()
      );
      expect(vizingPadAccount.engineAdmin.toBase58()).to.equal(
        initParams.engineAdmin.toBase58()
      );
      expect(vizingPadAccount.feeReceiver.toBase58()).to.equal(
        initParams.feeReceiver.toBase58()
      );
      expect(vizingPadAccount.stationAdmin.toBase58()).to.equal(
        initParams.stationAdmin.toBase58()
      );
      expect(vizingPadAccount.gasPoolAdmin.toBase58()).to.equal(
        initParams.gasPoolAdmin.toBase58()
      );
      expect(vizingPadAccount.registeredValidator.toBase58()).to.equal(
        initParams.registeredValidator.toBase58()
      );
      expect(vizingPadAccount.bump).to.equal(vizingPadBump);

      expect(vizingPadAccount.isPaused).to.equal(false);
    }

    {
      try {
        await vizingProgram.methods
          .initializeVizingPad(initParams)
          .accounts({
            vizing: vizingPadSettings,
            vizingAuthority: vizingAuthority,
            payer: provider.wallet.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .rpc();
        throw new Error("should not come here");
      } catch (error) {
        expect(error.transactionMessage).to.be.eql(
          "Transaction simulation failed: Error processing Instruction 0: custom program error: 0x0"
        );
        console.log("could not initialize vizing pad twice");
      }
    }
  });

  it("modify Vizing Pad", async () => {
    const modifyParams = {
      owner: provider.wallet.publicKey,
      feeReceiver: feeReceiverKeyPair.publicKey,
      engineAdmin: engineAdminKeyPairs.map((keypair) => keypair.publicKey)[0],
      gasPoolAdmin: gasPoolAdminKeyPair.publicKey,
      stationAdmin: stationAdminKeyPair.publicKey,
      trustedRelayers: trustedRelayerKeyPairs.map(
        (keypair) => keypair.publicKey
      ),
      registeredValidator: anchor.web3.Keypair.generate().publicKey,
      isPaused: false,
    };

    {
      try {
        const fakeOwner = anchor.web3.Keypair.generate();
        await vizingProgram.methods
          .modifySettings(modifyParams)
          .accounts({
            owner: fakeOwner.publicKey,
            vizing: vizingPadSettings,
          })
          .signers([fakeOwner])
          .rpc();
        throw new Error("should not come here");
      } catch (error) {
        expect(error.error.errorMessage).to.equal("Unauthorized: Not Owner");
      }
    }

    {
      const tx = await vizingProgram.methods
        .modifySettings(modifyParams)
        .accounts({
          owner: provider.wallet.publicKey,
          vizing: vizingPadSettings,
        })
        .rpc();
      console.log(`modify: ${tx}`);

      const vizingPadAccount =
        await vizingProgram.account.vizingPadSettings.fetch(vizingPadSettings);

      expect(vizingPadAccount.owner.toBase58()).to.equal(
        provider.wallet.publicKey.toBase58()
      );
      expect(vizingPadAccount.feeReceiver.toBase58()).to.equal(
        modifyParams.feeReceiver.toBase58()
      );
      expect(vizingPadAccount.engineAdmin.toBase58()).to.equal(
        modifyParams.engineAdmin.toBase58()
      );
      expect(vizingPadAccount.stationAdmin.toBase58()).to.equal(
        modifyParams.stationAdmin.toBase58()
      );
      expect(vizingPadAccount.registeredValidator.toBase58()).to.equal(
        modifyParams.registeredValidator.toBase58()
      );
    }
  });

  it("grant fee collector", async () => {
    const newFeeCollector = anchor.web3.Keypair.generate();

    {
      try {
        const fakeGasPoolAdmin = anchor.web3.Keypair.generate();
        await vizingProgram.methods
          .grantFeeCollector(newFeeCollector.publicKey)
          .accounts({
            gasPoolAdmin: fakeGasPoolAdmin.publicKey,
            vizing: vizingPadSettings,
          })
          .signers([fakeGasPoolAdmin])
          .rpc();
        throw new Error("should not come here");
      } catch (error) {
        expect(error.error.errorMessage).to.equal(
          "Unauthorized: Not Gas Pool Admin"
        );
      }
    }

    {
      const tx = await vizingProgram.methods
        .grantFeeCollector(newFeeCollector.publicKey)
        .accounts({
          gasPoolAdmin: gasPoolAdminKeyPair.publicKey,
          vizing: vizingPadSettings,
        })
        .signers([gasPoolAdminKeyPair])
        .rpc();
      console.log(`grant fee collector: ${tx}`);

      let vizingPadAccount =
        await vizingProgram.account.vizingPadSettings.fetch(vizingPadSettings);

      expect(vizingPadAccount.feeReceiver.toBase58()).to.equal(
        newFeeCollector.publicKey.toBase58()
      );

      await vizingProgram.methods
        .grantFeeCollector(feeReceiverKeyPair.publicKey)
        .accounts({
          gasPoolAdmin: gasPoolAdminKeyPair.publicKey,
          vizing: vizingPadSettings,
        })
        .signers([gasPoolAdminKeyPair])
        .rpc();

      vizingPadAccount = await vizingProgram.account.vizingPadSettings.fetch(
        vizingPadSettings
      );

      expect(vizingPadAccount.feeReceiver.toBase58()).to.equal(
        feeReceiverKeyPair.publicKey.toBase58()
      );
    }
  });

  it("Launch", async () => {
    const message = {
      mode: 5,
      targetContract: anchor.web3.Keypair.generate().publicKey,
      executeGasLimit: new anchor.BN(6),
      maxFeePerGas: new anchor.BN(7),
      signature: Buffer.from("1234"),
    };

    const launchParams = {
      erliestArrivalTimestamp: new anchor.BN(1),
      latestArrivalTimestamp: new anchor.BN(2),
      relayer: provider.wallet.publicKey,
      sender: provider.wallet.publicKey,
      value: new anchor.BN(3),
      destChainid: new anchor.BN(4),
      additionParams: Buffer.alloc(0),
      message: message,
    };

    {
      try {
        await vizingProgram.methods
          .launch(launchParams)
          .accounts({
            feePayer: provider.wallet.publicKey,
            messageAuthority: provider.wallet.publicKey,
            vizing: vizingPadSettings,
            feeCollector: anchor.web3.Keypair.generate().publicKey,
          })
          .rpc();
        throw new Error("should not come here");
      } catch (error) {
        expect(error.error.errorMessage).to.equal(
          "Unauthorized: Fee Collector Invalid"
        );
      }
    }

    {
      const feeReceiverBalanceBefore = await provider.connection.getBalance(
        feeReceiverKeyPair.publicKey
      );

      const tx = await vizingProgram.methods
        .launch(launchParams)
        .accounts({
          feePayer: provider.wallet.publicKey,
          messageAuthority: provider.wallet.publicKey,
          vizing: vizingPadSettings,
          feeCollector: feeReceiverKeyPair.publicKey,
        })
        .rpc();
      console.log(`launch: ${tx}`);

      const feeReceiverBalanceAfter = await provider.connection.getBalance(
        feeReceiverKeyPair.publicKey
      );

      console.log(
        `feeReceiverBalanceBefore: ${
          feeReceiverBalanceBefore / anchor.web3.LAMPORTS_PER_SOL
        }, feeReceiverBalanceAfter: ${
          feeReceiverBalanceAfter / anchor.web3.LAMPORTS_PER_SOL
        }`
      );

      expect(feeReceiverBalanceAfter).to.greaterThan(feeReceiverBalanceBefore);
    }
  });

  it("Landing", async () => {
    await vizingAppProgram.methods.initialize().rpc();
    await vizingAppMockProgram.methods.initialize().rpc();

    const targetContract = vizingAppMockProgram.programId;

    const message = {
      mode: 5,
      targetContract: targetContract,
      executeGasLimit: new anchor.BN(6),
      maxFeePerGas: new anchor.BN(7),
      signature: Buffer.alloc(0),
    };

    const landingParams = {
      messageId: Buffer.from([
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
      ]),
      erliestArrivalTimestamp: new anchor.BN(1),
      latestArrivalTimestamp: new anchor.BN(2),
      srcChainid: new anchor.BN(3),
      srcTxHash: Buffer.from([
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
      ]),
      srcContract: anchor.web3.Keypair.generate().publicKey,
      srcChainNonce: new anchor.BN(4),
      sender: provider.wallet.publicKey,
      value: new anchor.BN(5),
      additionParams: Buffer.alloc(0),
      message: message,
    };

    {
      const tx = await vizingProgram.methods
        .landing(landingParams)
        .accounts({
          relayer: trustedRelayerKeyPairs[0].publicKey,
          vizing: vizingPadSettings,
          vizingAuthority: vizingAuthority,
          targetContract: targetContract,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .remainingAccounts([
          {
            pubkey: vizingAuthority,
            isSigner: false,
            isWritable: true,
          },
          {
            pubkey: targetContract,
            isSigner: false,
            isWritable: true,
          },
        ])
        .signers([trustedRelayerKeyPairs[0]])
        .rpc();
      console.log(`landing tx: ${tx}`);
    }
  });
});
