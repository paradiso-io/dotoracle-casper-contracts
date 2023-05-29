require('dotenv').config()
const fs = require('fs');

const { utils, helpers } = require('casper-js-client-helper')
const { getDeploy } = require('./utils')

const {
  CLValueBuilder,
  Keys,
  CLPublicKey,
  RuntimeArgs} = require('casper-js-sdk')
let key = require('./keys.json').keyTestnet

const {
  installContract,
  createRecipientAddress
} = helpers;

const {
  NODE_ADDRESS,
  CHAIN_NAME,
  WASM_PATH
} = process.env
let paymentAmount = '170000000000'

let privateKeyBuffer = Keys.Secp256K1.parsePrivateKey(Uint8Array.from(Buffer.from(key, 'hex')), 'raw')
let publicKey = Keys.Secp256K1.privateToPublicKey(Uint8Array.from(privateKeyBuffer))
let KEYS = new Keys.Secp256K1.parseKeyPair(publicKey, Uint8Array.from(privateKeyBuffer), 'raw')

console.log('pubkey', KEYS.accountHex())
const contract_key_name = "nft_bridge_custodian_8"
// let contract_owner = "02038df1cff6b55615858b1acd2ebcce98db164f88cf88919c7b045268571cc49cb7" // MPC
const contract_owner = KEYS.accountHex()
// let dev = "017e80955a6d493a4a4b9f1b5dd23d2edcdc2c8b00fcd9689f2f735f501bd088c5" // ABB
const dev = KEYS.accountHex()
const test = async () => {
  const runtimeArgs = RuntimeArgs.fromMap({
    contract_name: CLValueBuilder.string(contract_key_name),
    contract_owner: createRecipientAddress(CLPublicKey.fromHex(contract_owner)), //MPC
    dev: createRecipientAddress(CLPublicKey.fromHex(dev)), // ABB
    disable_older_version_or_not: CLValueBuilder.bool(true)
  });

  let hash = await installContract(
    CHAIN_NAME,
    NODE_ADDRESS,
    KEYS,
    runtimeArgs,
    paymentAmount,
    WASM_PATH
  );

  console.log(`... Contract installation deployHash: ${hash}`)

  await getDeploy(NODE_ADDRESS, hash)

  let accountInfo = await utils.getAccountInfo(NODE_ADDRESS, KEYS.publicKey)

  console.log(`... Contract installed successfully.`)

  console.log(`... Account Info: `)
  console.log(JSON.stringify(accountInfo, null, 2))
  fs.writeFileSync('deploy/contractinfo.json', JSON.stringify(accountInfo, null, 2));
}
test()
