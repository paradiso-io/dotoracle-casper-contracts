require("dotenv").config();
const { getDeploy } = require("./utils");
let key = require('./keys.json').keyTestnet
const {
  Keys,
  CLByteArray
} = require("casper-js-sdk");
const configed = require("./config.json")
const CWeb3 = require('casper-web3')

const { NODE_ADDRESS, CHAIN_NAME } =
  process.env;

let privateKeyBuffer = Keys.Secp256K1.parsePrivateKey(Uint8Array.from(Buffer.from(key, 'hex')), 'raw')
let publicKey = Keys.Secp256K1.privateToPublicKey(Uint8Array.from(privateKeyBuffer))
let KEYS = new Keys.Secp256K1.parseKeyPair(publicKey, Uint8Array.from(privateKeyBuffer), 'raw')
console.log("pubkey", KEYS.accountHex());
const test = async () => {
  const nft_contract = configed.nftPackageHash
  const nftContractHash = await CWeb3.Contract.getActiveContractHash(nft_contract, CHAIN_NAME)
  const contract = await CWeb3.Contract.createInstanceWithRemoteABI(nftContractHash, NODE_ADDRESS, CHAIN_NAME)

  let hash = await contract.contractCalls.mint.makeDeployAndSend({
    keys: KEYS,
    args: {
      tokenOwner: KEYS.publicKey
    },
    paymentAmount: '20000000000'
  })

  console.log(`... Contract installation deployHash: ${hash}`);

  await getDeploy(NODE_ADDRESS, hash);

  console.log(`... Contract installed successfully.`);
};

test();
