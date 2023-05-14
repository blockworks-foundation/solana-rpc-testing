import { Keypair } from "@solana/web3.js";
import * as fs from "fs";

export function getKeypairFromFile(filePath: String): Keypair {
  return Keypair.fromSecretKey(
    Uint8Array.from(
      JSON.parse(
        process.env.KEYPAIR || fs.readFileSync(filePath.toString(), "utf-8")
      )
    )
  );
}
