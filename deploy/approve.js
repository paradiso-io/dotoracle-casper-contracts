require("dotenv").config();
let contractInfo = require("./contractinfo.json");
const { sleep, getDeploy } = require("./utils");
let key = require('./keys.json').key
const {
  Keys,
  CLAccountHash,
  CLPublicKey,
  CLByteArray
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
  "3a100016a814263b64223357b169ac94ff84d1fd5826efaf1935543287066fc1";
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

  let bridgeContract = "00bc38235189835c213f1c331c4322f54d56d4584418c7b5b7b71c92812b354d"
  bridgeContract = new CLByteArray(
    Uint8Array.from(Buffer.from(bridgeContract, "hex"))
  );


  let hash = await cep78.approveForAll({
    keys: KEYS,
    operator: bridgeContract
  });

  console.log(`... Contract installation deployHash: ${hash}`);

  await getDeploy(NODE_ADDRESS, hash);

  console.log(`... Contract installed successfully.`);
};

test();
