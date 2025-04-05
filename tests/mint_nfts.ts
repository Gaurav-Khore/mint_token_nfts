import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MintNfts } from "../target/types/mint_nfts";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";
import { min } from "bn.js";
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddress, TOKEN_PROGRAM_ID } from "@solana/spl-token";

describe("mint_nfts", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const payer = provider.wallet as anchor.Wallet;

  const program = anchor.workspace.MintNfts as Program<MintNfts>;

  const [mint, bumps] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("mint")],
   program.programId
  );

  const [tokenAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("token")],
    program.programId
  );

  it("Create Mint!", async () => {
    // Add your test here.
    console.log(mint.toBase58());
    const tx = await program.methods.createMint().accounts({
      signer: payer.publicKey,
      mint: mint
    })
    .rpc();
    console.log("Your transaction signature", tx);
    
  });

  it("Create Token Account",async () => {

    console.log("Token Account = > ", tokenAccount.toBase58());

    const tx = await program.methods.createTokenAccount().accounts({
      signer: payer.publicKey,
      mint: mint,
      tokenAccount: tokenAccount
    }).rpc();

    console.log("Trandacation is completed .. ",tx);
  });

  it("Create Associated Token Account => ",async () => {

    const userTokenAccount = await getAssociatedTokenAddress(
      mint,
      payer.publicKey,
      false,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );
    const tx = await program.methods.createTokenAccountAssociated().accounts({
      mint: mint,
      tokenAccount: userTokenAccount
    }).rpc();

    console.log("Associated Successful , ",tx);
  })
});
