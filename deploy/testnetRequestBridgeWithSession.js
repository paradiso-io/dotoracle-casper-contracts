require("dotenv").config();
const { getDeploy, genRanHex } = require("./utils");
let key = require('./keys.json').keyTestnet
const CWeb3 = require('casper-web3')
const configed = require("./config.json")

const { Keys, CLByteArray, CLValueBuilder } = require("casper-js-sdk");

const { NODE_ADDRESS, CHAIN_NAME, CEP78_SESSION_WASM_PATH } =
  process.env;

let nft_bridge_contract_package = configed.bridgePackageHash

let privateKeyBuffer = Keys.Secp256K1.parsePrivateKey(Uint8Array.from(Buffer.from(key, 'hex')), 'raw')
let publicKey = Keys.Secp256K1.privateToPublicKey(Uint8Array.from(privateKeyBuffer))
let KEYS = new Keys.Secp256K1.parseKeyPair(publicKey, Uint8Array.from(privateKeyBuffer), 'raw')

const test = async () => {
  const bridgeContractHash = await CWeb3.Contract.getActiveContractHash(nft_bridge_contract_package, CHAIN_NAME)
  const hash = await CWeb3.Contract.makeInstallContractAndSend({
    keys: KEYS,
    args: {
      token_ids: CLValueBuilder.list(configed.tokenIds.map(e => CLValueBuilder.u64(e))),
      nft_package_hash: CLValueBuilder.key(new CLByteArray(Uint8Array.from(Buffer.from(configed.nftPackageHash, "hex")))),
      to_chainid: CLValueBuilder.u256(43113),
      identifier_mode: CLValueBuilder.u8(0),
      receiver_address: CLValueBuilder.string("0x37E6C45c1B1D4EeF4F50e52cCAbc5AdeC15995B1"),
      bridge_contract_hash: CLValueBuilder.key(new CLByteArray(Uint8Array.from(Buffer.from(bridgeContractHash, "hex"))))
    },
    paymentAmount: '30000000000',
    chainName: CHAIN_NAME,
    nodeAddress: NODE_ADDRESS,
    wasmPath: CEP78_SESSION_WASM_PATH
  })

  console.log(`... Contract installation deployHash: ${hash}`);

  await getDeploy(NODE_ADDRESS, hash);

  console.log(`... Contract installed successfully.`);
};

test();
