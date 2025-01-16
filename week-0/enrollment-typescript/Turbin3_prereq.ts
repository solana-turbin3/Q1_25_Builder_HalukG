// programs/Turbin3_prereq.ts

import { Idl } from "@coral-xyz/anchor";

/**
 * This IDL is copied verbatim from the on-chain version you shared,
 * so it aligns perfectly with the deployed code on Devnet.
 */
export const IDL: Idl = {
  // The on-chain program ID:
  address: "WBAQSygkwMox2VuWKU133NxFrpDZUBdvSBeaBEue2Jq",

  metadata: {
    name: "wba_prereq",
    version: "0.1.0",
    spec: "0.1.0",
    description: "Created with Anchor",
  },

  instructions: [
    {
      name: "complete",
      // IMPORTANT: matches the chain's 8-byte instruction discriminator
      discriminator: [0, 77, 224, 147, 136, 25, 88, 76],
      accounts: [
        {
          // "signer" is your wallet
          name: "signer",
          writable: true,
          signer: true
        },
        {
          // "prereq" is the PDA derived from seeds ["prereq", signer]
          name: "prereq",
          writable: true
        },
        {
          // Must be named "system_program" EXACTLY
          name: "system_program",
          address: "11111111111111111111111111111111"
        }
      ],
      args: [
        {
          name: "github",
          type: "bytes"
        }
      ]
    },
    {
      name: "update",
      discriminator: [219, 200, 88, 176, 158, 63, 253, 127],
      accounts: [
        {
          name: "signer",
          writable: true,
          signer: true
        },
        {
          name: "prereq",
          writable: true
        },
        {
          name: "system_program",
          address: "11111111111111111111111111111111"
        }
      ],
      args: [
        {
          name: "github",
          type: "bytes"
        }
      ]
    }
  ],

  accounts: [
    {
      name: "Q2Prereq2024",
      discriminator: [210, 203, 168, 103, 251, 233, 204, 6]
    }
  ],

  errors: [
    {
      code: 6000,
      name: "InvalidGithubAccount",
      msg: "Invalid Github account"
    }
  ],

  types: [
    {
      name: "Q2Prereq2024",
      type: {
        kind: "struct",
        fields: [
          { name: "github", type: "bytes" },
          { name: "key", type: "pubkey" }
        ]
      }
    }
  ],

  constants: []
};
