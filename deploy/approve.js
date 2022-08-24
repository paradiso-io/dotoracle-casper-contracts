require("dotenv").config();
let contractInfo = require("./contractinfo.json");
const { sleep, getDeploy } = require("./utils");
let key = require('./keys.json').key
const {
  Keys,
  CLAccountHash,
  CLPublicKey,
} = require("casper-js-sdk");

const CEP78 = require("./cep78");

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
const test = async () => {
  let cep78 = new CEP78(nft_contract, NODE_ADDRESS, CHAIN_NAME);
  await cep78.init();

  let hash = await cep78.approve(
    KEYS,
    CLPublicKey.fromHex("0106ca7c39cd272dbf21a86eeb3b36b7c26e2e9b94af64292419f7862936bca2ca"),
    //new CLAccountHash(Uint8Array.from(Buffer.from(nft_bridge_contract, "hex"))),
    32,
    "1000000000"
  );

  console.log(`... Contract installation deployHash: ${hash}`);

  await getDeploy(NODE_ADDRESS, hash);

  console.log(`... Contract installed successfully.`);
};

test();
