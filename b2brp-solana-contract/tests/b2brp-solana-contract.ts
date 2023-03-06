import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { B2brpSolanaContract } from "../target/types/b2brp_solana_contract";

describe("b2brp-solana-contract", () => {

  const testTitle = "B2BRP Solana Contract";
  const testNftSymbol = "B2BRP";
  const tetNftUri = "https://b2brp.com";



  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.B2brpSolanaContract as Program<B2brpSolanaContract>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
