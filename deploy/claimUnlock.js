require("dotenv").config();
let contractInfo = require("./contractinfo.json");
const { CasperContractClient, helpers } = require("casper-js-client-helper");
const { getDeploy } = require("./utils");
const { createRecipientAddress } = helpers;
const sdk = require('../index')
let key = require('./keys.json').key

const { CLValueBuilder, Keys, RuntimeArgs, CLByteArrayBytesParser, CLByteArray, CLAccountHash } = require("casper-js-sdk");

const { NODE_ADDRESS, EVENT_STREAM_ADDRESS, CHAIN_NAME, WASM_PATH } =
  process.env;

let privateKeyPem = `
-----BEGIN PRIVATE KEY-----
${key}
-----END PRIVATE KEY-----
`; // abb key

let nft_bridge_contract = "6c6dd4f31ed62a5dcc7cdb750b1b45e7acb92fdd1a14d0990b2b8f3ccf719e0d"

let nft_contract =
  "3a100016a814263b64223357b169ac94ff84d1fd5826efaf1935543287066fc1"; // NFT CEP78
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

const test = async () => {
  let bridge = await sdk.NFTBridge.createInstance(nft_bridge_contract, NODE_ADDRESS, CHAIN_NAME)
  let hash = await bridge.claimUnlockNft({
    keys: KEYS,
  })

  console.log(`... Contract installation deployHash: ${hash}`);

  await getDeploy(NODE_ADDRESS, hash);

  console.log(`... Contract installed successfully.`);
};

test();
