import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { UserManager } from "../target/types/user_manager";

// Pad Buffer
//
// This function pads a buffer with zeros to the specified length.
function padBuffer(buffer: Buffer, length: number): Buffer {
  const zeroPad = Buffer.alloc(length - buffer.length, 0);
  return Buffer.concat([buffer, zeroPad]);
}

describe("user-manager", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.UserManager as Program<UserManager>;

  it("Is initialized!", async () => {
    // Add your test here.
    const create_user_data = {
      id: anchor.web3.PublicKey.unique(),
      signing_key: anchor.web3.PublicKey.unique(),
    };
    // convert data to Uint8Array
    try {
      const tx = await program.methods
        .createUser(Buffer.from(JSON.stringify(create_user_data)))
        .rpc();
      console.log(tx);
    } catch (error: anchor.AnchorError | any) {
      // console.log(await error.getLogs());
      console.log(error);
    }
  });
});
