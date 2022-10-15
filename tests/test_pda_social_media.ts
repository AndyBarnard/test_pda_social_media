import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { TestPdaSocialMedia } from "../target/types/test_pda_social_media";

describe("test_pda_social_media", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TestPdaSocialMedia as Program<TestPdaSocialMedia>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
