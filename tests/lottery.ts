import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Lottery } from "../target/types/lottery";

describe("lottery", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const wallet = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.lottery as Program<Lottery>;

  it("it should init!", async () => {
    const initConfixTx = await program.methods
      .initializeConfig(new anchor.BN(1000), new anchor.BN(0), new anchor.BN(10000))
      .instruction();
    const blockHashwithCtx = await provider.connection.getLatestBlockhash();
    const tx = new anchor.web3.Transaction({
      blockhash: blockHashwithCtx.blockhash,
      feePayer: wallet.publicKey,
      lastValidBlockHeight: blockHashwithCtx.lastValidBlockHeight,
    }).add(initConfixTx);
    const signature = await anchor.web3.sendAndConfirmTransaction(
      provider.connection,
      tx,
      [wallet.payer]
    );
    console.log(signature);
  });
});
