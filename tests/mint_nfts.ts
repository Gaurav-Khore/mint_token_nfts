import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MintNfts } from "../target/types/mint_nfts";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";
import { BN, min } from "bn.js";
import { ASSOCIATED_TOKEN_PROGRAM_ID, getAssociatedTokenAddress, getAssociatedTokenAddressSync, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Keypair, PublicKey } from "@solana/web3.js";

describe("mint_nfts", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const payer = provider.wallet as anchor.Wallet;

  const program = anchor.workspace.MintNfts as Program<MintNfts>;

  const [mint, bumps] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("mint1")],
   program.programId
  );

  const [tokenAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("token1")],
    program.programId
  );

  const tokenMetadataProgram = new PublicKey('metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s');
  const tokenProgram = new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

  const [metadataAccount] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from('metadata'),tokenMetadataProgram.toBuffer(),mint.toBuffer()],
    tokenMetadataProgram
  );

  const spl_token = {
    name: 'G Coin',
    symbol: 'GOLDSOL',
    uri: 'https://raw.githubusercontent.com/Gaurav-Khore/mint_token_nfts/refs/heads/main/.assests/spl-token.json'
  }

  const nft_token = {
    name: 'G Coin NFT',
    symbol: 'NFTGOLDSOL',
    uri: 'https://raw.githubusercontent.com/Gaurav-Khore/mint_token_nfts/refs/heads/main/.assests/nft-token.json'
  }

  it("Create Mint!", async () => {
    // Add your test here.
    console.log(mint.toBase58());
    const tx = await program.methods.createMint(spl_token.name,spl_token.symbol,spl_token.uri).accounts({
      signer: payer.publicKey,
      mint: mint,
      metadataAccount: metadataAccount,
      tokenMetadataProgram: tokenMetadataProgram,
      tokenProgram: tokenProgram
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
    console.log("user token acount",userTokenAccount);
    const tx = await program.methods.createTokenAccountAssociated().accounts({
      mint: mint,
      tokenAccount: userTokenAccount
    }).rpc();

    console.log("Associated Successful , ",tx);
  });

  it("Mint G Coins" , async () => {
    console.log("Mint Account = ",mint.toBase58());

    const amount = new anchor.BN(10000000);
    const userTokenAccount = await getAssociatedTokenAddress(
      mint,
      payer.publicKey,
      false,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );
    const tx = await program.methods.miningToken(amount)
    .accounts({
      mint: mint,
      tokenAccount: userTokenAccount
    }).rpc();

    console.log("Mining completed in tx = ",tx);
  });


  it("Transfer Tokens", async () => {

    const senderTokenAccount = await getAssociatedTokenAddress(
      mint,
      payer.publicKey,
      false,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    console.log("Snder Token account = ",senderTokenAccount);
    console.log("Receiver Token Account = ",tokenAccount);
    console.log("Mint = ", mint);

    let amount = new anchor.BN(100000)

    const tx = await program.methods.transferToken(amount)
    .accounts({
      mint: mint,
      senderTokenAccount: senderTokenAccount,
      receiverTokenAccount: tokenAccount
    }).rpc();

    console.log("Trnasfer is successfull , please check tx = ",tx);

  });


  it("Create NFT" , async () => {
    console.log("Creating nft");

    const mintKeyPair = new Keypair();
    console.log("mint token for nft ",mintKeyPair.publicKey.toBase58());

    const userTokenAccount = getAssociatedTokenAddressSync(
      mintKeyPair.publicKey,
      payer.publicKey
    );

    const [metadataAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('metadata'),tokenMetadataProgram.toBuffer(),mintKeyPair.publicKey.toBuffer()],
      tokenMetadataProgram
    );

    const [editionAccount] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('metadata'),tokenMetadataProgram.toBuffer(),mintKeyPair.publicKey.toBuffer(),Buffer.from("edition")],
      tokenMetadataProgram
    );

    console.log("User Token Account = ",userTokenAccount);

    const tx = await program.methods.mintNfts(nft_token.name,nft_token.symbol,nft_token.uri)
    .accounts({
      mintAccount: mintKeyPair.publicKey,
      associatedTokenAccount: userTokenAccount,
      payer: payer.publicKey,
      metadataAccount: metadataAccount,
      editionAccount: editionAccount,
      tokenMetadataProgram: tokenMetadataProgram,
      tokenProgram: tokenProgram
    }).signers([mintKeyPair]).rpc();

    console.log("nft created successfully , tx = ",tx);

  })

});
