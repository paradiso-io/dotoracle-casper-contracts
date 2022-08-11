const {
  utils,
  helpers,
  CasperContractClient,
} = require("casper-js-client-helper");

const { setClient, contractSimpleGetter } = helpers;

const NFTBridge = class {
  constructor(contractHash, nodeAddress, chainName) {
    this.contractHash = contractHash.startsWith("hash-")
      ? contractHash.slice(5)
      : contractHash;
    this.nodeAddress = nodeAddress;
    this.chainName = chainName;
    this.contractClient = new CasperContractClient(nodeAddress, chainName);
  }

  async init() {
    const { contractPackageHash, namedKeys } = await setClient(
      this.nodeAddress,
      this.contractHash,
      ["request_ids"]
    );
    this.contractPackageHash = contractPackageHash;
    this.contractClient.chainName = this.chainName;
    this.contractClient.contractHash = this.contractHash;
    this.contractClient.contractPackageHash = this.contractPackageHash;
    this.contractClient.nodeAddress = this.nodeAddress;
    /* @ts-ignore */
    this.namedKeys = namedKeys;
  }

  async contractOwner() {
    return await contractSimpleGetter(this.nodeAddress, this.contractHash, [
      "contract_owner",
    ]);
  }

  async requestIndex() {
    return await contractSimpleGetter(this.nodeAddress, this.contractHash, [
      "request_index",
    ]);
  }

  async getIndexFromRequestId(requestId) {
    try {
      const itemKey = requestId.toString();
      const result = await utils.contractDictionaryGetter(
        this.nodeAddress,
        itemKey,
        this.namedKeys.requestIds
      );
      return result.val.data.data.toString();
    } catch (e) {
      throw e;
    }
  }
};

module.exports = { NFTBridge };
