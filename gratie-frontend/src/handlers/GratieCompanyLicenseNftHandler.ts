// This is a handler for the GratieCompanyLicenseNft anchor program

import { Buffer as BufferPolyfill } from 'buffer'
declare const Buffer: typeof BufferPolyfill;
globalThis.Buffer = BufferPolyfill

import { GratieCompanyLicenseNft } from "@/types/gratie_company_license_nft";
import { AnchorProvider, Program, Wallet } from "@project-serum/anchor";
import { clusterApiUrl, Connection, Keypair, PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY, Transaction } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, createAssociatedTokenAccountInstruction, getAssociatedTokenAddress, createInitializeMintInstruction, MINT_SIZE } from '@solana/spl-token'
import idl from '../idl/gratie_company_license_nft.json';

const programID = new PublicKey('66oo66h8fF83T5ttnBLHbuaBesy6WRNFDhpsVPQUvsis');
const network = clusterApiUrl('devnet');

const opts = {
  preFlightCommitment: 'processed',
};

export class GratieCompanyLicenseNftHandler {
  private static getProvider() {
    const { solana } = window as any;

    const conn = new Connection(network, opts.preFlightCommitment as any);
    const provider = new AnchorProvider(conn, solana, opts.preFlightCommitment as any)
    return provider;
  }

  // Creates a company license
  static async createCompanyLicenseNft(companyName: string) {
    const provider = this.getProvider();
    const wallet = provider.wallet as Wallet;

    const program: Program<GratieCompanyLicenseNft> = new Program(idl as any, programID, provider);

    const TOKEN_METADATA_PROGRAM_ID = new PublicKey(
      "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
    );


    const lamports: number =
      await program.provider.connection.getMinimumBalanceForRentExemption(
        MINT_SIZE
      );


    const getMetadata = async (
      mint: PublicKey
    ): Promise<PublicKey> => {
      return (
        await PublicKey.findProgramAddress(
          [
            Buffer.from("metadata"),
            TOKEN_METADATA_PROGRAM_ID.toBuffer(),
            mint.toBuffer(),
          ],
          TOKEN_METADATA_PROGRAM_ID
        )
      )[0];
    };

    const getMasterEdition = async (
      mint: PublicKey
    ): Promise<PublicKey> => {
      return (
        await PublicKey.findProgramAddress(
          [
            Buffer.from("metadata"),
            TOKEN_METADATA_PROGRAM_ID.toBuffer(),
            mint.toBuffer(),
            Buffer.from("edition"),
          ],
          TOKEN_METADATA_PROGRAM_ID
        )
      )[0];
    };

    const mintKey: Keypair = Keypair.generate();

    const NftTokenAccount = await getAssociatedTokenAddress(
      mintKey.publicKey,
      wallet.publicKey
    );
    console.log("NFT Account: ", NftTokenAccount.toBase58());

    const mint_tx = new Transaction().add(
      SystemProgram.createAccount({
        fromPubkey: wallet.publicKey,
        newAccountPubkey: mintKey.publicKey,
        space: MINT_SIZE,
        programId: TOKEN_PROGRAM_ID,
        lamports,
      }),
      createInitializeMintInstruction(
        mintKey.publicKey,
        0,
        wallet.publicKey,
        wallet.publicKey
      ),
      createAssociatedTokenAccountInstruction(
        wallet.publicKey,
        NftTokenAccount,
        wallet.publicKey,
        mintKey.publicKey
      )
    );

    const res = await (program as any).provider.sendAndConfirm(mint_tx, [mintKey]);

    console.log(
      await program.provider.connection.getParsedAccountInfo(mintKey.publicKey)
    );

    console.log("Account: ", res);
    console.log("Mint key: ", mintKey.publicKey.toString());
    console.log("User: ", wallet.publicKey.toString());

    const metadataAddress = await getMetadata(mintKey.publicKey);
    const masterEdition = await getMasterEdition(mintKey.publicKey);

    console.log("Metadata address: ", metadataAddress.toBase58());
    console.log("MasterEdition: ", masterEdition.toBase58());

    // Here we call the transaction
    const tx = await program.methods.mintNft(
      mintKey.publicKey,
      "https://raw.githubusercontent.com/mucks/b2b-rewards-platform-solana/main/gratie-company-license-nft/assets/company-license-sample.json",
      companyName,
    )
      .accounts({
        mintAuthority: wallet.publicKey,
        mint: mintKey.publicKey,
        tokenAccount: NftTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
        metadata: metadataAddress,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        payer: wallet.publicKey,
        systemProgram: SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
        masterEdition: masterEdition,
      },
      )
      .rpc();
    console.log("Your transaction signature", tx);
  }

}