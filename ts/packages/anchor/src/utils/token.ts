import { PublicKey } from "@solarti/web3.js";

export const TOKEN_PROGRAM_ID = new PublicKey(
  "Token4Q2B47VCdUy8u3rSTMMk2bGA1k7eN8qfKSzdiM"
);
export const ASSOCIATED_PROGRAM_ID = new PublicKey(
  "ATAccPjxdgWfJKKN4PmfJ55FbEDEwD8zJUwVjuL9MuHy"
);

export function associatedAddress({
  mint,
  owner,
}: {
  mint: PublicKey;
  owner: PublicKey;
}): PublicKey {
  return PublicKey.findProgramAddressSync(
    [owner.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), mint.toBuffer()],
    ASSOCIATED_PROGRAM_ID
  )[0];
}
