require("dotenv").config();
const { helpers, utils } = require("casper-js-client-helper");
const { getDeploy, genRanHex } = require("./utils");
const sdk = require('../index')
let key = require('./keys.json').keyTestnet
const CWeb3 = require('casper-web3')
const configed = require("./config.json")

const { Keys, CLByteArray } = require("casper-js-sdk");

const { NODE_ADDRESS, CHAIN_NAME } =
  process.env;

let nft_bridge_contract_package = configed.bridgePackageHash

let nft_contract = configed.nftPackageHash
let privateKeyBuffer = Keys.Secp256K1.parsePrivateKey(Uint8Array.from(Buffer.from(key, 'hex')), 'raw')
let publicKey = Keys.Secp256K1.privateToPublicKey(Uint8Array.from(privateKeyBuffer))
let KEYS = new Keys.Secp256K1.parseKeyPair(publicKey, Uint8Array.from(privateKeyBuffer), 'raw')

const test = async () => {
  const bridgeContractHash = await CWeb3.Contract.getActiveContractHash(nft_bridge_contract_package, CHAIN_NAME)
  const contract = await CWeb3.Contract.createInstanceWithRemoteABI(bridgeContractHash, NODE_ADDRESS, CHAIN_NAME)
  const hash = await contract.contractCalls.requestBridgeNft.makeDeployAndSend({
      keys: KEYS,
      args: {
        tokenIds: configed.tokenIds,
        nftPackageHash: new CLByteArray(
          Uint8Array.from(Buffer.from(nft_contract, "hex"))
        ),
        toChainid: 43113,
        identifierMode: 0,
        receiverAddress: "0xbf26a30547a7dda6e86fc3C33396F28FFf6902c3",
        requestId: genRanHex()
      },
      paymentAmount: '40000000000'
  })

  console.log(`... Contract installation deployHash: ${hash}`);

  await getDeploy(NODE_ADDRESS, hash);

  console.log(`... Contract installed successfully.`);
};

test();
