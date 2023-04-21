require("dotenv").config();
let contractInfo = require("./contractinfo.json");
const { CasperContractClient, helpers } = require("casper-js-client-helper");
const { getDeploy } = require("./utils");
const { createRecipientAddress } = helpers;
const sdk = require('../index')
let key = require('./keys.json').key

const { CLValueBuilder, Keys, RuntimeArgs, CLByteArrayBytesParser, CLByteArray, CLKey,CLPublicKey, CLAccountHash } = require("casper-js-sdk");

const { NODE_ADDRESS, EVENT_STREAM_ADDRESS, CHAIN_NAME, WASM_PATH } =
  process.env;

let privateKeyPem = `
-----BEGIN PRIVATE KEY-----
${key}
-----END PRIVATE KEY-----
`; // abb key

let nft_bridge_contract = "dc4d6de2bfbfaf5b422bfc32fb154da122f5d6374e1919ddfc6a1cc38fc7323b"
console.log("nft_bridge_contract: ", nft_bridge_contract)
let nft_contract =
  "a198be397f8d07f184ec0cf1b5a23e28e713d59ff2bd2f6c25d98f66883d61af"
// "68d05b72593981f73f5ce7ce5dcac9033aa0ad4e8c93b773f8b939a18c0bbc3b";
//"805347b595cc24814f0d50482069a1dba24f9bfb2823c6e900386f147f25754b";
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
  console.log("done instance bridge")
  let cep78 = await sdk.CEP78.createInstance(nft_contract, NODE_ADDRESS, CHAIN_NAME)
  console.log("done instance cep78")
  const contracthashbytearray = new CLByteArray(Uint8Array.from(Buffer.from("dc4d6de2bfbfaf5b422bfc32fb154da122f5d6374e1919ddfc6a1cc38fc7323b", 'hex')));
  const nftContractHash = new CLKey(contracthashbytearray);

  let owner = "017e80955a6d493a4a4b9f1b5dd23d2edcdc2c8b00fcd9689f2f735f501bd088c5"

  let hashApprove = await cep78.approveForAll({
    keys: KEYS,
    owner: owner,
    operator: nftContractHash
  })
  console.log(`... Contract installation deployHash: ${hashApprove}`);

  await getDeploy(NODE_ADDRESS, hashApprove);


  let hash = await bridge.requestBridgeNFT({
    keys: KEYS,
    tokenIds: [0],
    nftContractHash: nft_contract,
    toChainId: 43113,
    identifierMode: 0,
    receiverAddress: "0xbf26a30547a7dda6e86fc3C33396F28FFf6902c3",
  })

  console.log(`... Contract installation deployHash: ${hash}`);

  await getDeploy(NODE_ADDRESS, hash);

  console.log(`... Contract installed successfully.`);
};

test();
