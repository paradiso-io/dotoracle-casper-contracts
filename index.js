const {
  utils,
  helpers,
  CasperContractClient,
} = require("casper-js-client-helper");

const CEP78 = require("./cep78");
const {
  CLValueBuilder,
  CLKey,
  CasperClient,
  CLByteArray,
  RuntimeArgs,
  CLAccountHash,
  DeployUtil,
} = require("casper-js-sdk");
const { DEFAULT_TTL } = require("casper-js-client-helper/dist/constants");

const { setClient, contractSimpleGetter, createRecipientAddress } = helpers;

const sleep = (ms) => {
  return new Promise((resolve) => setTimeout(resolve, ms));
};

const getDeploy = async (NODE_URL, deployHash) => {
  const client = new CasperClient(NODE_URL);
  let i = 300;
  while (i != 0) {
    const [deploy, raw] = await client.getDeploy(deployHash);
    if (raw.execution_results.length !== 0) {
      // @ts-ignore
      if (raw.execution_results[0].result.Success) {
        return deploy;
      } else {
        // @ts-ignore
        throw Error(
          "Contract execution: " +
          // @ts-ignore
          raw.execution_results[0].result.Failure.error_message
        );
      }
    } else {
      i--;
      await sleep(1000);
      continue;
    }
  }
  throw Error("Timeout after " + i + "s. Something's wrong");
};

const genRanHex = (size = 64) =>
  [...Array(size)]
    .map(() => Math.floor(Math.random() * 16).toString(16))
    .join("");
const NFTBridge = class {
  constructor(contractHash, nodeAddress, chainName) {
    this.contractHash = contractHash.startsWith("hash-")
      ? contractHash.slice(5)
      : contractHash;
    this.nodeAddress = nodeAddress;
    this.chainName = chainName;
    this.contractClient = new CasperContractClient(nodeAddress, chainName);
  }

  static async createInstance(contractHash, nodeAddress, chainName) {
    let bridge = new NFTBridge(contractHash, nodeAddress, chainName);
    await bridge.init();
    console.log("NameKey: ", bridge.namedKeys)
    return bridge;
  }

  async init() {
    console.log("intializing", this.nodeAddress, this.contractHash);
    const { contractPackageHash, namedKeys } = await setClient(
      this.nodeAddress,
      this.contractHash,
      ["request_ids"]
    );
    console.log("done");
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
      return result;
    } catch (e) {
      throw e;
    }
  }
  async requestBridgeNFT({
    keys,
    tokenIds,
    nftContractHash,
    toChainId,
    identifierMode,
    paymentAmount,
    ttl,
    receiverAddress,

  }) {
    if (!paymentAmount) {
      paymentAmount = paymentAmount ? paymentAmount : "3000000000";
      ttl = ttl ? ttl : DEFAULT_TTL;
    }

    if (identifierMode == undefined) {
      let nftContract = new CEP78(
        nftContractHash,
        this.nodeAddress,
        this.chainName
      );
      await nftContract.init();
      identifierMode = await nftContract.identifierMode();
    }
    nftContractHash = nftContractHash.startsWith("hash-")
      ? nftContractHash.slice(5)
      : nftContractHash;
    console.log("nftContractHash", nftContractHash);
    nftContractHash = new CLByteArray(
      Uint8Array.from(Buffer.from(nftContractHash, "hex"))
    );
    let runtimeArgs = {};
    if (identifierMode == 0) {
      tokenIds = tokenIds.map((e) => CLValueBuilder.u64(e));

      runtimeArgs = RuntimeArgs.fromMap({
        token_ids: CLValueBuilder.list(tokenIds),
        identifier_mode: CLValueBuilder.u8(identifierMode),
        nft_contract_hash: createRecipientAddress(nftContractHash),
        to_chainid: CLValueBuilder.u256(toChainId),
        request_id: CLValueBuilder.string(genRanHex()),
        receiver_address: CLValueBuilder.string(receiverAddress),
      })
    } else {
      console.log("TOkenIDS A: ", tokenIds)
      tokenIds = tokenIds.map((e) => CLValueBuilder.string(e));
      console.log("TOkenIDS B: ", tokenIds)
      runtimeArgs = RuntimeArgs.fromMap({
        token_hashes: CLValueBuilder.list(tokenIds),
        identifier_mode: CLValueBuilder.u8(identifierMode),
        nft_contract_hash: createRecipientAddress(nftContractHash),
        to_chainid: CLValueBuilder.u256(toChainId),
        request_id: CLValueBuilder.string(genRanHex()),
        receiver_address: CLValueBuilder.string(receiverAddress),
      });
    }

    console.log("sending");
    let trial = 5;
    while (true) {
      try {
        let hash = await this.contractClient.contractCall({
          entryPoint: "request_bridge_nft",
          keys: keys,
          paymentAmount,
          runtimeArgs,
          cb: (deployHash) => {
            console.log("deployHash", deployHash);
          },
          ttl,
        });

        return hash;
      } catch (e) {
        trial--
        if (trial == 0) {
          throw e;
        }
        console.log('waiting 2 seconds')
        await sleep(3000)
      }
    }
  }

  async unlockNFT({
    keys,
    tokenIds,
    nftContractHash,
    fromChainId,
    identifierMode,
    paymentAmount,
    ttl,
    receiverAddress,

  }) {
    if (!paymentAmount) {
      paymentAmount = paymentAmount ? paymentAmount : "3000000000";
      ttl = ttl ? ttl : DEFAULT_TTL;
    }

    if (identifierMode == undefined) {
      let nftContract = new CEP78(
        nftContractHash,
        this.nodeAddress,
        this.chainName
      );
      await nftContract.init();
      identifierMode = await nftContract.identifierMode();
    }
    nftContractHash = nftContractHash.startsWith("hash-")
      ? nftContractHash.slice(5)
      : nftContractHash;
    console.log("nftContractHash", nftContractHash);
    nftContractHash = new CLByteArray(
      Uint8Array.from(Buffer.from(nftContractHash, "hex"))
    );
    // let ownerAccountHashByte = new CLAccountHash(Uint8Array.from(
    //   Buffer.from(receiverAddress, 'hex'),
    // ))
    // let ownerAccountHashByte = Uint8Array.from(
    //   Buffer.from(receiverAddress, 'hex'),
    // )
    // const receiverAccounthash = new CLAccountHash(
    //   ownerAccountHashByte
    // );
    // const receiverKey = new CLKey(receiverAccounthash);
    // console.log("token_owner_to_casper:  ", receiverKey)
    let recipientAccountHashByte = Uint8Array.from(
      Buffer.from(receiverAddress, 'hex'),
    )
    const accounthash2 = new CLAccountHash(
      recipientAccountHashByte
    );
    const token_owner_to_casper = new CLKey(accounthash2);
    console.log("token_owner_to_casper:  ", token_owner_to_casper)



    let runtimeArgs = {};
    if (identifierMode == 0) {
      tokenIds = tokenIds.map((e) => CLValueBuilder.u64(e));
      runtimeArgs = RuntimeArgs.fromMap({
        token_ids: CLValueBuilder.list([CLValueBuilder.u64(5)]), //tokenIds
        identifier_mode: CLValueBuilder.u8(identifierMode),
        receiver_address: token_owner_to_casper,
        nft_contract_hash: createRecipientAddress(nftContractHash),
        from_chainid: CLValueBuilder.u256(fromChainId),
         
      })
    } else {
      console.log("TOkenIDS A: ", tokenIds)
      tokenIds = tokenIds.map((e) => CLValueBuilder.string(e));
      console.log("TOkenIDS B: ", tokenIds)
      runtimeArgs = RuntimeArgs.fromMap({
        token_hashes: CLValueBuilder.list(tokenIds),
        identifier_mode: CLValueBuilder.u8(identifierMode),
        nft_contract_hash: createRecipientAddress(nftContractHash),
        from_chainid: CLValueBuilder.u256(fromChainId),
        // receiver_address: createRecipientAddress(ownerAccountHashByte),
      });
    }

    console.log("sending");
    let trial = 5;
    while (true) {
      try {
        let hash = await this.contractClient.contractCall({
          entryPoint: "unlock_nft",
          keys: keys,
          paymentAmount,
          runtimeArgs,
          cb: (deployHash) => {
            console.log("deployHash", deployHash);
          },
          ttl,
        });
        //   let deploy = await DeployUtil.makeDeploy(
        //     new DeployUtil.DeployParams(
        //       keys.publicKey,
        //         "casper-test"
        //     ),
        //     DeployUtil.ExecutableDeployItem.newStoredContractByHash(
        //         Uint8Array.from(Buffer.from("0776a9154e189c84ac947e128ac155e12f92853fc581148bb2ac90c2466f6285", "hex")),
        //         "unlock_nft",
        //         runtimeArgs,
        //     ),
        //     DeployUtil.standardPayment(2000000000)
        // );

        // console.log("Deploy: ", deploy)
        // const client = new CasperClient(this.nodeAddress)
        // deploy = await client.signDeploy(deploy, keys);

        // let deployHash = await client.putDeploy(deploy);
        // console.log("deployHash: ", deployHash)

        return hash;
      } catch (e) {
        trial--
        if (trial == 0) {
          throw e;
        }
        console.log('waiting 2 seconds')
        await sleep(3000)
      }
    }
  }
};

const DTOWrappedNFT = class extends CEP78 {
  constructor(contractHash, nodeAddress, chainName, namedKeysList = []) {
    super(contractHash, nodeAddress, chainName, namedKeysList)
  }

  static async createInstance(contractHash, nodeAddress, chainName, namedKeysList = []) {
    let wNFT = new DTOWrappedNFT(contractHash, nodeAddress, chainName, namedKeysList);
    await wNFT.init();
    return wNFT;
  }

  async init() {
    console.log("intializing", this.nodeAddress, this.contractHash);
    this.namedKeysList.push("request_ids")
    const { contractPackageHash, namedKeys } = await setClient(
      this.nodeAddress,
      this.contractHash,
      this.namedKeysList
    );
    console.log("done");
    this.contractPackageHash = contractPackageHash;
    this.contractClient.chainName = this.chainName;
    this.contractClient.contractHash = this.contractHash;
    this.contractClient.contractPackageHash = this.contractPackageHash;
    this.contractClient.nodeAddress = this.nodeAddress;
    /* @ts-ignore */
    this.namedKeys = namedKeys;
  }

  async dto_minter() {
    return await contractSimpleGetter(this.nodeAddress, this.contractHash, [
      "dto_minter",
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
      return result;
    } catch (e) {
      throw e;
    }
  }

  async mint({
    keys,
    tokenIds,
    metadatas,
    mintid,
    paymentAmount,
    ttl,
    tokenOwner
  }) {
    if (!paymentAmount) {
      paymentAmount = paymentAmount ? paymentAmount : "1000000000";
      ttl = ttl ? ttl : DEFAULT_TTL;
    }
    let identifierMode = await this.identifierMode()

    let runtimeArgs = {};
    metadatas = metadatas.map(e => CLValueBuilder.string(e))
    if (identifierMode == 0) {
      tokenIds = tokenIds.map((e) => CLValueBuilder.u64(e));
      runtimeArgs = RuntimeArgs.fromMap({
        token_ids: CLValueBuilder.list(tokenIds),
        token_meta_datas: CLValueBuilder.list(metadatas),
        mint_id: CLValueBuilder.string(mintid),
        token_owner: createRecipientAddress(tokenOwner)
      });
    } else {
      tokenIds = tokenIds.map((e) => CLValueBuilder.string(e));
      runtimeArgs = RuntimeArgs.fromMap({
        token_hashes: CLValueBuilder.list(tokenIds),
        token_meta_datas: CLValueBuilder.list(metadatas),
        mint_id: CLValueBuilder.string(mintid),
        token_owner: createRecipientAddress(tokenOwner)
      });
    }

    console.log("sending");
    let trial = 5;
    while (true) {
      try {
        let hash = await this.contractClient.contractCall({
          entryPoint: "mint",
          keys: keys,
          paymentAmount,
          runtimeArgs,
          cb: (deployHash) => {
            console.log("deployHash", deployHash);
          },
          ttl,
        });

        return hash;
      } catch (e) {
        trial--
        if (trial == 0) {
          throw e;
        }
        console.error('waiting 2 seconds', e)
        await sleep(3000)
      }
    }
  }

  async requestBridgeNFT({
    keys,
    tokenIds,
    toChainId,
    paymentAmount,
    ttl,
    receiverAddress
  }) {
    if (!paymentAmount) {
      paymentAmount = paymentAmount ? paymentAmount : "1000000000";
      ttl = ttl ? ttl : DEFAULT_TTL;
    }
    let identifierMode = await this.identifierMode()
    let runtimeArgs = {};
    if (identifierMode == 0) {
      tokenIds = tokenIds.map((e) => CLValueBuilder.u64(e));
      runtimeArgs = RuntimeArgs.fromMap({
        token_ids: CLValueBuilder.list(tokenIds),
        to_chainid: CLValueBuilder.u256(toChainId),
        request_id: CLValueBuilder.string(genRanHex()),
        receiver_address: CLValueBuilder.string(receiverAddress)
      });
    } else {
      tokenIds = tokenIds.map((e) => CLValueBuilder.string(e));
      runtimeArgs = RuntimeArgs.fromMap({
        token_hashes: CLValueBuilder.list(tokenIds),
        to_chainid: CLValueBuilder.u256(toChainId),
        request_id: CLValueBuilder.string(genRanHex()),
        receiver_address: CLValueBuilder.string(receiverAddress)
      });
    }

    let trial = 5;
    while (true) {
      try {
        let hash = await this.contractClient.contractCall({
          entryPoint: "request_bridge_back",
          keys: keys,
          paymentAmount,
          runtimeArgs,
          cb: (deployHash) => {
            console.log("deployHash", deployHash);
          },
          ttl,
        });

        return hash;
      } catch (e) {
        trial--
        if (trial == 0) {
          throw e;
        }
        console.log('waiting 2 seconds ' + e)
        await sleep(3000)
      }
    }
  }
};

module.exports = { NFTBridge, genRanHex, DTOWrappedNFT, CEP78 };
