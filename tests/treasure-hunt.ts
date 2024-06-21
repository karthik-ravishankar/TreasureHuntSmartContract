import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { TreasureHunt } from "../target/types/treasure_hunt";

describe("treasure-hunt", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TreasureHunt as Program<TreasureHunt>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});