require("dotenv").config();
const { getDeploy } = require("./utils");
let key = require('./keys.json').keyTestnet
const {
  Keys,
  CLByteArray
} = require("casper-js-sdk");
const CWeb3 = require('casper-web3')
const { NODE_ADDRESS, CHAIN_NAME } =
  process.env;
const configed = require("./config.json")
let privateKeyBuffer = Keys.Secp256K1.parsePrivateKey(Uint8Array.from(Buffer.from(key, 'hex')), 'raw')
let publicKey = Keys.Secp256K1.privateToPublicKey(Uint8Array.from(privateKeyBuffer))
let KEYS = new Keys.Secp256K1.parseKeyPair(publicKey, Uint8Array.from(privateKeyBuffer), 'raw')
console.log("pubkey", KEYS.accountHex());
const test = async () => {
  const nft_contract = configed.nftPackageHash
  const nftContractHash = await CWeb3.Contract.getActiveContractHash(nft_contract, CHAIN_NAME)
  let bridgeContractPackage = configed.bridgePackageHash
  const contract = await CWeb3.Contract.createInstanceWithRemoteABI(nftContractHash, NODE_ADDRESS, CHAIN_NAME)
  let bridgeContract = await CWeb3.Contract.getActiveContractHash(bridgeContractPackage, CHAIN_NAME)
  bridgeContract = new CLByteArray(
    Uint8Array.from(Buffer.from(bridgeContract, "hex"))
  );
  console.log(Object.keys(contract.contractCalls))
  let hash = await contract.contractCalls.setApprovalForAll.makeDeployAndSend({
    keys: KEYS,
    args: {
      operator: bridgeContract,
      approveAll: true
    },
    paymentAmount: "2000000000"
  })

  console.log(`... Contract installation deployHash: ${hash}`);

  await getDeploy(NODE_ADDRESS, hash);

  console.log(`... Contract installed successfully.`);
};

test();
