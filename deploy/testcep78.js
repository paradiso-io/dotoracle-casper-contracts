require("dotenv").config();
const { CLAccountHash, CLPublicKey } = require("casper-js-sdk");
let CEP78 = require("./cep78");
let contractHash =
  //"f22f266ea7171e2e3c7c21266c8b7f0da2ddee8b2357ca85339af73a4018d374";
  //"805347b595cc24814f0d50482069a1dba24f9bfb2823c6e900386f147f25754b";
  "52f370db3aeaa8c094e73a3aa581c85abc775cc52605e9cd9364cae0501ce645";
let contractInfo = require("./contractinfo.json");
let nft_bridge_contract = contractInfo.namedKeys
  .filter((e) => e.name == "dotoracle_nft_bridge_contract")[0]
  .key.slice(5);
const { NODE_ADDRESS, EVENT_STREAM_ADDRESS, CHAIN_NAME, WASM_PATH } =
  process.env;
async function main() {
  let contract = new CEP78(contractHash, NODE_ADDRESS, CHAIN_NAME);
  await contract.init();
  // let identifier_mode = await contract.identifierMode();
  // console.log("identifier_mode", identifier_mode.toString());

  //   let ownerOf = await cep78.getOwnerOf(31);
  //   console.log("ownerOf", ownerOf);

  //   let balanceOf = await cep78.balanceOf("3bdcc50ce1e1e0119d4901b686c65c66b63cc17e5fa5da2299e332c545ec23c6")
  //   console.log('balanceOf', balanceOf.toString())

  //   let burntTokens = await cep78.burntTokens(31);
  //   console.log("burntTokens", burntTokens);

  //   let metadata = await cep78.getTokenMetadata(31);
  //   console.log("metadata", metadata);

  //   let operator = await cep78.getOperator(31);
  //   console.log("operator", operator);

  try {
    let metadata = await contract.getOwnedTokens(CLPublicKey.fromHex("017e80955a6d493a4a4b9f1b5dd23d2edcdc2c8b00fcd9689f2f735f501bd088c5"))
    console.log("metadata", metadata);
  } catch (e) {
    console.error(e)
  }
}

main();
