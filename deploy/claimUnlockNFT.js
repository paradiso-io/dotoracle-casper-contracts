require("dotenv").config();
const { getDeploy } = require("./utils");
let key = require('./keys.json').keyTestnet
const CWeb3 = require('casper-web3')

const { Keys } = require("casper-js-sdk");
const configed = require("./config.json")
const { NODE_ADDRESS, CHAIN_NAME } =
  process.env;

let nft_bridge_contract_package = configed.bridgePackageHash

let privateKeyBuffer = Keys.Secp256K1.parsePrivateKey(Uint8Array.from(Buffer.from(key, 'hex')), 'raw')
let publicKey = Keys.Secp256K1.privateToPublicKey(Uint8Array.from(privateKeyBuffer))
let KEYS = new Keys.Secp256K1.parseKeyPair(publicKey, Uint8Array.from(privateKeyBuffer), 'raw')

const test = async () => {
  const bridgeContractHash = await CWeb3.Contract.getActiveContractHash(nft_bridge_contract_package, CHAIN_NAME)
  const contract = await CWeb3.Contract.createInstanceWithRemoteABI(bridgeContractHash, NODE_ADDRESS, CHAIN_NAME)
  const hash = await contract.contractCalls.claimUnlockNft.makeDeployAndSend({
      keys: KEYS,
      args: {
      },
      paymentAmount: '40000000000'
  })

  console.log(`... Contract installation deployHash: ${hash}`);

  await getDeploy(NODE_ADDRESS, hash);

  console.log(`... Contract installed successfully.`);
};

test();
