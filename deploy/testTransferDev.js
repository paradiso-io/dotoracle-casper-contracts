const {
  DeployUtil,
  CasperClient,
  RuntimeArgs,
  CLString,
  CLPublicKey,
  CLByteArray,
  CLKey,
  Keys,
  CLAccountHash,
  CLBool,
} = require("casper-js-sdk");

const { getDeploy } = require("./utils");
let key = require('./keys.json').key
let NODE_ADDRESS = require("./keys.json").NODE_ADDRESS
const { CasperContractClient, helpers } = require("casper-js-client-helper");
const { createRecipientAddress } = helpers;


const main = async () => {

  //Step 1: Set casper node client
  const client = new CasperClient(NODE_ADDRESS);
  console.log(NODE_ADDRESS)

  //Step 2: Set user key pair


  console.log("A")
  let privateKeyPem = `
-----BEGIN PRIVATE KEY-----
${key}
-----END PRIVATE KEY-----
`;
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

  console.log("B")
  //cep78 contract-hash
  //const hash1 = "e6e087a685a66f884d2c3a0f6252cbe57aaf259d2c56a88041e72161bd4e8047"
  //const hash1 = "ae396053d9b477cb53319fd8f6e3b2907d5bd7ef8e8ba622df648529ecb05526"
  //const hash1 = "805347b595cc24814f0d50482069a1dba24f9bfb2823c6e900386f147f25754b"  // This is Token_id ordial contract
  //const hash1 = "ed51bb0f987248a90ca15d4c8fffa85bad9b3d6c0223cbd2f2395de41f31ce6c"
  //const hash1 = "52f370db3aeaa8c094e73a3aa581c85abc775cc52605e9cd9364cae0501ce645" // This is Token_id hash contract
  //const hash1 = "9acbd5338e21b9d4251309784c09557d04de2fbe6fe49b865676027831794743" // For edit function on contract
  //const hash1 = "13270d53a99e783131add35acf4a38520ec407db3cc2cb2ca738f9bd24e861ff" // add correct minter
  //const hash1 = "c7b4fa010b49683bcbf0c888bedd71ef18cf699ddeb8c52eb50ad70cff6f69c9" // DTO- CASPER  bridge contract

  //const hash1 = "506e9d33b80cecb2cfd1906988a092ba062b0df68525c2e1be7f32cb92bdb71d"
  const hash1 = "c6a6375c9ca9149fd80e35076fa500b88e29ca401262fae79ee7bb13b8adf176"
  //=== token_owner: Key ===
  // const hexString =
  //     "017e80955a6d493a4a4b9f1b5dd23d2edcdc2c8b00fcd9689f2f735f501bd088c5";

  //  const hexString =
  //      "020389f6a966469e202fe6ff01f1ccf5a2ffcd02b96b69fb6cde1c1d32ae4d120688";
  // const hexString = "020261207299a7d59261d28a0780b92f76b5caff3ee2e3f767d7cd832e269c181767"

  //const hexString = "0158cdd1af07c27a6180ade7f09389357370fe7247ab62fc4d866a03141746c68d"
  const hexString = "017e80955a6d493a4a4b9f1b5dd23d2edcdc2c8b00fcd9689f2f735f501bd088c5" //abb account 
  const accounthash = new CLAccountHash(
    CLPublicKey.fromHex(hexString).toAccountHash()
  );
  // const token_owner = accounthash;
  const token_owner = new CLKey(accounthash);
  // console.log("tokenowner: ", token_owner)

  console.log("C")

  const wrapped_token_adr = "ed51bb0f987248a90ca15d4c8fffa85bad9b3d6c0223cbd2f2395de41f31ce6c"
  const contracthashbytearray = new CLByteArray(Uint8Array.from(Buffer.from(wrapped_token_adr, 'hex')));
  const contracthash = createRecipientAddress(contracthashbytearray);
  console.log("contracthash: ", contracthash)
  let is_wrapped_token = new CLBool(true)

  let deploy = DeployUtil.makeDeploy(
    new DeployUtil.DeployParams(
      KEYS.publicKey,
      "casper-test",
    ),
    DeployUtil.ExecutableDeployItem.newStoredContractByHash(
      Uint8Array.from(Buffer.from(hash1, 'hex')), // DTO CASPER BRIDGE contract
      "transfer_dev",
      RuntimeArgs.fromMap({
        "new_dev": token_owner, // token address
      })
    ),
    // DeployUtil.ExecutableDeployItem.newStoredContractByHash(
    //   Uint8Array.from(Buffer.from(hash1, 'hex')), // DTO CASPER BRIDGE contract
    //   "transfer_owner",
    //   RuntimeArgs.fromMap({
    //     "contract_owner": token_owner, 
    //   })
    // ),
    DeployUtil.standardPayment(2000000000)
  );
  console.log("D")

  deploy = client.signDeploy(deploy, KEYS);


  console.log("SIGNED");

  let deployHash = await client.putDeploy(deploy);

  console.log("E")

  console.log(`deploy hash = ${deployHash}`);

  const resultx = await getDeploy(NODE_ADDRESS, deployHash)

};

main();

