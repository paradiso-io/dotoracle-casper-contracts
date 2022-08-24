require("dotenv").config();
const { CLAccountHash, CLPublicKey } = require("casper-js-sdk");
let { CEP78, DTOWrappedNFT } = require("./index");
let contractHash =
  "f22f266ea7171e2e3c7c21266c8b7f0da2ddee8b2357ca85339af73a4018d374";

const { NODE_ADDRESS, EVENT_STREAM_ADDRESS, CHAIN_NAME, WASM_PATH } =
  process.env;
async function main() {
  let contract = await DTOWrappedNFT.createInstance(contractHash, NODE_ADDRESS, CHAIN_NAME);
  //await contract.init();
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
    let metadata = await contract.getTokenMetadata("0000000000000000000000000000000000000000000000000000000000000000")
    console.log("metadata", metadata);
  } catch (e) {
    console.error(e)
  }
}

main();
