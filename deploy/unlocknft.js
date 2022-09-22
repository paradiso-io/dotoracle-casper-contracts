require("dotenv").config();
let contractInfo = require("./contractinfo.json");
const { CasperContractClient, helpers } = require("casper-js-client-helper");
const { sleep, getDeploy } = require("./utils");
const { createRecipientAddress } = helpers;
let key = require('./keys.json').key

const { CLValueBuilder, CLPublicKey, Keys, RuntimeArgs, CLByteArrayBytesParser, CLByteArray } = require("casper-js-sdk");

const { NODE_ADDRESS, EVENT_STREAM_ADDRESS, CHAIN_NAME, WASM_PATH } =
  process.env;

let privateKeyPem = `
-----BEGIN PRIVATE KEY-----
${key}
-----END PRIVATE KEY-----
`;

let nft_bridge_contract = contractInfo.namedKeys
  .filter((e) => e.name == "dotoracle_nft_bridge_contract")[0]
  .key.slice(5);

let nft_contract =
  "805347b595cc24814f0d50482069a1dba24f9bfb2823c6e900386f147f25754b";
let privateKeyBuffer = Keys.Ed25519.parsePrivateKey(
  Keys.Ed25519.readBase64WithPEM(privateKeyPem)
);
let publicKey = Keys.Ed25519.privateToPublicKey(
  Uint8Array.from(privateKeyBuffer)
);
let KEYS = new Keys.Ed25519.parseKeyPair(
  publicKey,
  Uint8Array.from(privateKeyBuffer)
);
console.log("pubkey", KEYS.accountHex());
let contract_key_name = "dotoracle_nft_bridge_contract";
let contract_owner = KEYS.accountHex();
const test = async () => {
  let contractClient = new CasperContractClient(NODE_ADDRESS, CHAIN_NAME);
  let nftContractHash = new CLByteArray(Uint8Array.from(Buffer.from(nft_contract, "hex")))
  console.log('nftContractHash', nftContractHash.clType().toString())
  const runtimeArgs = RuntimeArgs.fromMap({
    token_ids: CLValueBuilder.list([CLValueBuilder.u64(2)]),
    identifier_mode: CLValueBuilder.u8(0),
    nft_contract_hash: createRecipientAddress(nftContractHash),
    from_chainid: CLValueBuilder.u256(43113),
    target_key: createRecipientAddress(KEYS.publicKey)
  });

  contractClient.contractHash = nft_bridge_contract.startsWith("hash-")
    ? nft_bridge_contract.slice(5)
    : nft_bridge_contract;

  console.log(contractClient);

  let hash = await contractClient.contractCall({
    entryPoint: "unlock_nft",
    keys: KEYS,
    paymentAmount: "5000000000",
    runtimeArgs,
    cb: (deployHash) => {
      console.log("deployHash", deployHash);
    },
    ttl: 900000,
  });

  console.log(`... Contract installation deployHash: ${hash}`);

  //await getDeploy(NODE_ADDRESS, hash);

  console.log(`... Contract installed successfully.`);
};

test();
