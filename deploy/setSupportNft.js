require("dotenv").config();
const { helpers } = require("casper-js-client-helper");
const { getDeploy } = require("./utils");
const sdk = require('../index')
let key = require('./keys.json').keyTestnet

const { Keys } = require("casper-js-sdk");
const CWeb3 = require('casper-web3')
const { NODE_ADDRESS, CHAIN_NAME } =
  process.env;
const configed = require("./config.json")


let nft_bridge_contract = configed.bridgePackageHash

let nft_contract = configed.nftPackageHash
let privateKeyBuffer = Keys.Secp256K1.parsePrivateKey(Uint8Array.from(Buffer.from(key, 'hex')), 'raw')
let publicKey = Keys.Secp256K1.privateToPublicKey(Uint8Array.from(privateKeyBuffer))
let KEYS = new Keys.Secp256K1.parseKeyPair(publicKey, Uint8Array.from(privateKeyBuffer), 'raw')

const test = async () => {
  nft_bridge_contract = await CWeb3.Contract.getActiveContractHash(nft_bridge_contract, CHAIN_NAME)
  let bridge = await sdk.NFTBridge.createInstance(nft_bridge_contract, NODE_ADDRESS, CHAIN_NAME)
  let hash = await bridge.setSupportedToken({
    keys: KEYS,
    nftContractHash: nft_contract,
    isSupportedToken: true,
  })

  console.log(`... Contract installation deployHash: ${hash}`);

  await getDeploy(NODE_ADDRESS, hash);

  console.log(`... Contract installed successfully.`);
};

test();
