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

let nft_bridge_contract = "dc4d6de2bfbfaf5b422bfc32fb154da122f5d6374e1919ddfc6a1cc38fc7323b"

let nft_contract =
  "a198be397f8d07f184ec0cf1b5a23e28e713d59ff2bd2f6c25d98f66883d61af";
 //"52f370db3aeaa8c094e73a3aa581c85abc775cc52605e9cd9364cae0501ce645";
 //"44f244fb474431a20c4968d60550f790000d21785650c963f9ac5e02c126e1fb";
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
  let hash = await bridge.unlockNFT({
    keys: KEYS,
    tokenIds: [0],
    nftContractHash: nft_contract, 
    fromChainId: 43113,
    identifierMode: 0,
    receiverAddress: "55884917f4107a59e8c06557baee7fdada631af6d1c105984d196a84562854eb",
  })

  console.log(`... Contract installation deployHash: ${hash}`);

  await getDeploy(NODE_ADDRESS, hash);

  console.log(`... Contract installed successfully.`);
};

test();
