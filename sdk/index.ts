import { Program, AnchorProvider, web3 } from "@coral-xyz/anchor";
import idl from "./idl.json";

export class ExceefireClient {
  program: Program;

  constructor(provider: AnchorProvider) {
    this.program = new Program(idl as any, provider);
  }

  async openPosition(size: number, leverage: number) {
    const position = web3.Keypair.generate();

    await this.program.methods
      .openPosition(new web3.BN(size), leverage)
      .accounts({
        position: position.publicKey,
        user: this.program.provider.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([position])
      .rpc();

    return position.publicKey;
  }

  async closePosition(position: web3.PublicKey) {
    await this.program.methods
      .closePosition()
      .accounts({
        position,
        user: this.program.provider.publicKey,
      })
      .rpc();
  }
}
