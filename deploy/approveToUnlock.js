require("dotenv").config();
const { getDeploy, genRanHex } = require("./utils");
const sdk = require('../index')
let key = require('./keys.json').keyTestnet
const CWeb3 = require('casper-web3')

const { Keys, CLByteArray } = require("casper-js-sdk");

const { NODE_ADDRESS, CHAIN_NAME } =
  process.env;
const configed = require("./config.json")

let nft_bridge_contract_package = configed.bridgePackageHash

let nft_contract = configed.nftPackageHash
let privateKeyBuffer = Keys.Secp256K1.parsePrivateKey(Uint8Array.from(Buffer.from(key, 'hex')), 'raw')
let publicKey = Keys.Secp256K1.privateToPublicKey(Uint8Array.from(privateKeyBuffer))
let KEYS = new Keys.Secp256K1.parseKeyPair(publicKey, Uint8Array.from(privateKeyBuffer), 'raw')

const test = async () => {
  const bridgeContractHash = await CWeb3.Contract.getActiveContractHash(nft_bridge_contract_package, CHAIN_NAME)
  const contract = await CWeb3.Contract.createInstanceWithRemoteABI(bridgeContractHash, NODE_ADDRESS, CHAIN_NAME)
  const hash = await contract.contractCalls.approveToUnlockNft.makeDeployAndSend({
      keys: KEYS,
      args: {
        targetKey: KEYS.publicKey,
        unlockId: `0x7788d03de297137446ae4d66a5630d40064e8ec398305c7189f717e4b41914e2-43113-96945816564243-94-${nft_contract}-96945816564243`,
        tokenIds: ["656"],
        fromChainid: 43113,
        identifierMode: 0,
        nftPackageHash: new CLByteArray(
          Uint8Array.from(Buffer.from(nft_contract, "hex"))
        )
      },
      paymentAmount: '40000000000'
  })

  console.log(`... Contract installation deployHash: ${hash}`);

  await getDeploy(NODE_ADDRESS, hash);

  console.log(`... Contract installed successfully.`);
};

test();
