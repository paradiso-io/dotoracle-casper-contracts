require("dotenv").config();
let contractInfo = require("./contractinfo2.json");
const { CasperContractClient, helpers } = require("casper-js-client-helper");
const { getDeploy } = require("./utils");
const { createRecipientAddress } = helpers;
const sdk = require('../index')
let key = require('./keys.json').key

const { CLValueBuilder, Keys, RuntimeArgs, CLByteArrayBytesParser, CLByteArray } = require("casper-js-sdk");

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
  //"805347b595cc24814f0d50482069a1dba24f9bfb2823c6e900386f147f25754b";
 //"52f370db3aeaa8c094e73a3aa581c85abc775cc52605e9cd9364cae0501ce645";
 "44f244fb474431a20c4968d60550f790000d21785650c963f9ac5e02c126e1fb";
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
  let hash = await bridge.requestBridgeNFT({
    keys: KEYS,
    tokenIds: ["9b4105806029ce18702f36c0747eadb5eab21862f08b679772927b340b586b6e"],
    nftContractHash: nft_contract, 
    toChainId: 97,
    identifierMode: 1,
    receiverAddress: "0xe68485B880898B5C4Cde29d40A588846A5Ced582",
  })

  console.log(`... Contract installation deployHash: ${hash}`);

  await getDeploy(NODE_ADDRESS, hash);

  console.log(`... Contract installed successfully.`);
};

test();
