const {
    utils,
    helpers,
    CasperContractClient,
} = require("casper-js-client-helper");
const { DEFAULT_TTL } = require("casper-js-client-helper/dist/constants");

const { CLValueBuilder, RuntimeArgs } = require("casper-js-sdk");

const { setClient, contractSimpleGetter, createRecipientAddress } = helpers;

const CEP78 = class {
    constructor(contractHash, nodeAddress, chainName, namedKeysList = []) {
        this.contractHash = contractHash.startsWith("hash-")
            ? contractHash.slice(5)
            : contractHash;
        this.nodeAddress = nodeAddress;
        this.chainName = chainName;
        this.contractClient = new CasperContractClient(nodeAddress, chainName);
        this.namedKeysList = [
            "balances",
            "burnt_tokens",
            "metadata_cep78",
            "metadata_custom_validated",
            "metadata_nft721",
            "metadata_raw",
            "operator",
            "owned_tokens",
            "token_issuers",
            "token_owners",
        ];
        this.namedKeysList.push(...namedKeysList)
    }

    static async createInstance(contractHash, nodeAddress, chainName, namedKeysList = []) {
        let wNFT = new CEP78(contractHash, nodeAddress, chainName, namedKeysList);
        await wNFT.init();
        return wNFT;
    }

    NFTMetadataKind = {
        CEP78: 0,
        NFT721: 1,
        Raw: 2,
        CustomValidated: 3,
    };

    async init() {
        const { contractPackageHash, namedKeys } = await setClient(
            this.nodeAddress,
            this.contractHash,
            this.namedKeysList
        );
        this.contractPackageHash = contractPackageHash;
        this.contractClient.chainName = this.chainName
        this.contractClient.contractHash = this.contractHash
        this.contractClient.contractPackageHash = this.contractPackageHash
        this.contractClient.nodeAddress = this.nodeAddress
        /* @ts-ignore */
        this.namedKeys = namedKeys;
    }

    async identifierMode() {
        let mode = await contractSimpleGetter(this.nodeAddress, this.contractHash, [
            "identifier_mode",
        ]);
        return mode.toNumber()
    }

    async collectionName() {
        return await this.readContractField("collection_name");
    }

    async allowMinting() {
        return await this.readContractField("allow_minting");
    }

    async collectionSymbol() {
        return await this.readContractField("collection_symbol");
    }

    async contractWhitelist() {
        return await this.readContractField("contract_whitelist");
    }

    async holderMode() {
        return await this.readContractField("holder_mode");
    }

    async installer() {
        return await this.readContractField("installer");
    }

    async jsonSchema() {
        return await this.readContractField("json_schema");
    }

    async metadataMutability() {
        return await this.readContractField("metadata_mutability");
    }

    async mintingMode() {
        return await this.readContractField("minting_mode");
    }

    async nftKind() {
        return await this.readContractField("nft_kind");
    }

    async nftMetadataKind() {
        return await this.readContractField("nft_metadata_kind");
    }

    async numberOfMintedTokens() {
        return await this.readContractField("number_of_minted_tokens");
    }

    async ownershipMode() {
        return await this.readContractField("ownership_mode");
    }

    async receiptName() {
        return await this.readContractField("receipt_name");
    }

    async totalTokenSupply() {
        return await this.readContractField("total_token_supply");
    }

    async whitelistMode() {
        return await this.readContractField("whitelist_mode");
    }

    async readContractField(field) {
        return await contractSimpleGetter(this.nodeAddress, this.contractHash, [
            field,
        ]);
    }

    async getOperator(tokenId) {
        try {
            const itemKey = tokenId.toString();
            const result = await utils.contractDictionaryGetter(
                this.nodeAddress,
                itemKey,
                this.namedKeys.operator
            );
            return Buffer.from(result.val.data.data).toString("hex");
        } catch (e) {
            throw e;
        }
    }

    async getOwnerOf(tokenId) {
        try {
            const itemKey = tokenId.toString();
            const result = await utils.contractDictionaryGetter(
                this.nodeAddress,
                itemKey,
                this.namedKeys.tokenOwners
            );
            return Buffer.from(result.data).toString("hex");
        } catch (e) {
            throw e;
        }
    }

    async burntTokens(tokenId) {
        try {
            const itemKey = tokenId.toString();
            const result = await utils.contractDictionaryGetter(
                this.nodeAddress,
                itemKey,
                this.namedKeys.burntTokens
            );
            return result ? true : false;
        } catch (e) { }
        return false;
    }

    async getTokenMetadata(tokenId) {
        try {
            const itemKey = tokenId.toString();
            let nftMetadataKind = await this.nftMetadataKind();
            nftMetadataKind = parseInt(nftMetadataKind.toString());
            let result = null;
            if (nftMetadataKind == this.NFTMetadataKind.CEP78) {
                result = await utils.contractDictionaryGetter(
                    this.nodeAddress,
                    itemKey,
                    this.namedKeys.metadataCep78
                );
            } else if (nftMetadataKind == this.NFTMetadataKind.CustomValidated) {
                result = await utils.contractDictionaryGetter(
                    this.nodeAddress,
                    itemKey,
                    this.namedKeys.metadataCustomValidated
                );
            } else if (nftMetadataKind == this.NFTMetadataKind.NFT721) {
                result = await utils.contractDictionaryGetter(
                    this.nodeAddress,
                    itemKey,
                    this.namedKeys.metadataNft721
                );
            } else if (nftMetadataKind == this.NFTMetadataKind.Raw) {
                result = await utils.contractDictionaryGetter(
                    this.nodeAddress,
                    itemKey,
                    this.namedKeys.metadataRaw
                );
            }

            return result;
        } catch (e) {
            throw e;
        }
    }

    static getAccountItemKey(account) {
        let itemKey = "";
        if (typeof account == String) {
            itemKey = account.toString();
        } else {
            let key = createRecipientAddress(account);
            itemKey = Buffer.from(key.data.data).toString("hex");
        }
        return itemKey;
    }

    async getOwnedTokens(account) {
        try {
            let itemKey = CEP78.getAccountItemKey(account);
            const result = await utils.contractDictionaryGetter(
                this.nodeAddress,
                itemKey,
                this.namedKeys.ownedTokens
            );
            return result.map((e) => e.data);
        } catch (e) {
            throw e;
        }
    }

    async balanceOf(account) {
        try {
            let itemKey = CEP78.getAccountItemKey(account);
            const result = await utils.contractDictionaryGetter(
                this.nodeAddress,
                itemKey,
                this.namedKeys.balances
            );
            return result;
        } catch (e) {
            throw e;
        }
    }

    async approve(keys, operator, tokenId, paymentAmount, ttl) {
        let key = createRecipientAddress(operator);
        let identifierMode = await this.identifierMode();
        identifierMode = parseInt(identifierMode.toString());
        let runtimeArgs = {};
        if (identifierMode == 0) {
            runtimeArgs = RuntimeArgs.fromMap({
                token_id: CLValueBuilder.u64(parseInt(tokenId)),
                operator: key,
            });
        } else {
            runtimeArgs = RuntimeArgs.fromMap({
                token_hash: CLValueBuilder.string(tokenId),
                operator: key,
            });
        }

        return await this.contractClient.contractCall({
            entryPoint: "approve",
            keys: keys,
            paymentAmount: paymentAmount ? paymentAmount : "1000000000",
            runtimeArgs,
            cb: (deployHash) => { },
            ttl: ttl ? ttl : DEFAULT_TTL,
        });
    }

    async approveForAll(keys, operator, paymentAmount, ttl) {
        let key = createRecipientAddress(operator);
        let runtimeArgs = RuntimeArgs.fromMap({
            operator: key,
        });

        return await this.contractClient.contractCall({
            entryPoint: "set_approval_for_all",
            keys: keys,
            paymentAmount: paymentAmount ? paymentAmount : "1000000000",
            runtimeArgs,
            cb: (deployHash) => { },
            ttl: ttl ? ttl : DEFAULT_TTL,
        });
    }

    async burn(keys, tokenId, paymentAmount, ttl) {
        let identifierMode = await this.identifierMode();
        identifierMode = parseInt(identifierMode.toString());
        let runtimeArgs = {};
        if (identifierMode == 0) {
            runtimeArgs = RuntimeArgs.fromMap({
                token_id: CLValueBuilder.u64(parseInt(tokenId)),
            });
        } else {
            runtimeArgs = RuntimeArgs.fromMap({
                token_hash: CLValueBuilder.string(tokenId),
            });
        }

        return await this.contractClient.contractCall({
            entryPoint: "burn",
            keys: keys,
            paymentAmount: paymentAmount ? paymentAmount : "1000000000",
            runtimeArgs,
            cb: (deployHash) => { },
            ttl: ttl ? ttl : DEFAULT_TTL,
        });
    }

    async transfer(keys, source, recipient, tokenId, paymentAmount, ttl) {
        let identifierMode = await this.identifierMode();
        identifierMode = parseInt(identifierMode.toString());
        let runtimeArgs = {};
        if (identifierMode == 0) {
            runtimeArgs = RuntimeArgs.fromMap({
                token_id: CLValueBuilder.u64(parseInt(tokenId)),
                source_key: createRecipientAddress(source),
                target_key: createRecipientAddress(recipient),
            });
        } else {
            runtimeArgs = RuntimeArgs.fromMap({
                token_hash: CLValueBuilder.string(tokenId),
                source_key: createRecipientAddress(source),
                target_key: createRecipientAddress(recipient),
            });
        }

        return await this.contractClient.contractCall({
            entryPoint: "transfer",
            keys: keys,
            paymentAmount: paymentAmount ? paymentAmount : "1000000000",
            runtimeArgs,
            cb: (deployHash) => { },
            ttl: ttl ? ttl : DEFAULT_TTL,
        });
    }
};

module.exports = CEP78;
