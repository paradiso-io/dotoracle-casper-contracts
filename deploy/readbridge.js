require("dotenv").config();
const { CLAccountHash, CLPublicKey } = require("casper-js-sdk");
let Bridge = require("../index");
let contractHash =
  "805347b595cc24814f0d50482069a1dba24f9bfb2823c6e900386f147f25754b";
//    "52f370db3aeaa8c094e73a3aa581c85abc775cc52605e9cd9364cae0501ce645"
let contractInfo = require("./contractinfo.json");
let nft_bridge_contract = contractInfo.namedKeys
  .filter((e) => e.name == "dotoracle_nft_bridge_contract")[0]
  .key.slice(5);
const { NODE_ADDRESS, EVENT_STREAM_ADDRESS, CHAIN_NAME, WASM_PATH } =
  process.env;
async function main() {
  let bridge = await Bridge.NFTBridge.createInstance(
    nft_bridge_contract,
    NODE_ADDRESS,
    CHAIN_NAME
  );
  let requestData = await bridge.getIndexFromRequestId(
    "3b2e8d6c4c05c649f02001fec5ef41fba5f81685e0fd1d83676a0f640e3fff00"
  );
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

  //let account = new CLAccountHash(Uint8Array.from(Buffer.from("3bdcc50ce1e1e0119d4901b686c65c66b63cc17e5fa5da2299e332c545ec23c6", "hex")))
  // let account = CLPublicKey.fromHex(
  //   "0158cdd1af07c27a6180ade7f09389357370fe7247ab62fc4d866a03141746c68d"
  // );
  // try {
  //   let operator = await cep78.getOperator(32);
  //   console.log("operator", operator);
  // } catch (e) {}
  // let ownedTokens = await cep78.getOwnedTokens(account);
  requestData = JSON.parse(requestData)
  console.log("requestData", requestData.nft_contract_hash);
}

main();
