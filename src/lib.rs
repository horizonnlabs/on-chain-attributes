#![no_std]

extern crate alloc;
use alloc::string::ToString;

const IPFS_GATEWAY: &[u8] = "https://ipfs.io/ipfs/".as_bytes();
const METADATA_KEY_NAME: &[u8] = "metadata:".as_bytes();
const METADATA_FILE_EXTENSION: &[u8] = ".json".as_bytes();

const ROYALTIES: u64 = 100;
const NFT_AMOUNT: u64 = 1;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod token;

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi)]
pub struct NftAttributes<M: ManagedTypeApi> {
    pub background: ManagedBuffer<M>,
    pub skin: ManagedBuffer<M>,
    pub color: ManagedBuffer<M>,
    pub accessories: ManagedBuffer<M>,
    pub level: u16,
    pub metadata: ManagedBuffer<M>,
}

pub type AttributesAsMultiValue<M> =
    MultiValue6<u64, ManagedBuffer<M>, ManagedBuffer<M>, ManagedBuffer<M>, ManagedBuffer<M>, u16>;

#[elrond_wasm::contract]
pub trait OnChainAttributes: token::TokenModule {
    #[init]
    fn init(&self, image_cid: ManagedBuffer, metadata_cid: ManagedBuffer) {
        self.image_cid().set(&image_cid);
        self.metadata_cid().set(&metadata_cid);
    }

    #[only_owner]
    #[endpoint(createNft)]
    fn create_nft(&self, name: ManagedBuffer, token_attributes: AttributesAsMultiValue<Self::Api>) {
        require!(!self.nft_token().is_empty(), "Token is not issued");

        let (number, background, skin, color, accessories, level) = token_attributes.into_tuple();
        let metadata = self.build_metadata(number);

        let attributes = NftAttributes {
            background,
            skin,
            color,
            accessories,
            level,
            metadata,
        };
        let token_id = self.nft_token().get_token_id();

        let uri = self.build_uri(number);
        let mut uris = ManagedVec::new();
        uris.push(uri);

        self.send().esdt_nft_create::<NftAttributes<Self::Api>>(
            &token_id,
            &BigUint::from(NFT_AMOUNT),
            &name,
            &BigUint::from(ROYALTIES),
            &ManagedBuffer::new(),
            &attributes,
            &uris,
        );
    }

    #[only_owner]
    #[endpoint(createNftWithAttributesFromStorage)]
    fn create_nft_with_attributes_from_storage(&self, name: ManagedBuffer, number: u64) {
        require!(!self.nft_token().is_empty(), "Token is not issued");
        require!(
            !self.attributes(number).is_empty(),
            "On-chain attributes doesn't exist"
        );

        let token_id = self.nft_token().get_token_id();
        let on_chain_attributes = self.attributes(number).get();

        let uri = self.build_uri(number);
        let mut uris = ManagedVec::new();
        uris.push(uri);

        self.send().esdt_nft_create::<NftAttributes<Self::Api>>(
            &token_id,
            &BigUint::from(NFT_AMOUNT),
            &name,
            &BigUint::from(ROYALTIES),
            &ManagedBuffer::new(),
            &on_chain_attributes,
            &uris,
        );
    }

    #[only_owner]
    #[endpoint(updateAttributes)]
    fn update_attributes(&self, nft_nonce: u64) {
        let nft_attributes = self
            .nft_token()
            .get_token_attributes::<NftAttributes<Self::Api>>(nft_nonce);

        let new_attributes = NftAttributes {
            level: nft_attributes.level + 1,
            ..nft_attributes
        };
        let token_id = self.nft_token().get_token_id();

        self.send()
            .nft_update_attributes(&token_id, nft_nonce, &new_attributes);
    }

    #[only_owner]
    #[endpoint(mintNftWithNewUriAndAttributes)]
    fn mint_nft_with_new_uri_and_attributes(
        &self,
        nft_nonce: u64,
        name: ManagedBuffer,
        background: ManagedBuffer,
        skin: ManagedBuffer,
        color: ManagedBuffer,
        accessories: ManagedBuffer,
        new_image_uri: ManagedBuffer,
        new_metadata: ManagedBuffer,
    ) {
        let nft_attributes = self
            .nft_token()
            .get_token_attributes::<NftAttributes<Self::Api>>(nft_nonce);
        
        self.nft_token()
            .nft_burn(nft_nonce, &BigUint::from(NFT_AMOUNT));

        let new_attributes = NftAttributes {
            background,
            skin,
            color,
            accessories,
            level: nft_attributes.level + 1,
            metadata: new_metadata,
        };
        let token_id = self.nft_token().get_token_id();

        let mut uris: ManagedVec<ManagedBuffer> = ManagedVec::new();
        uris.push(new_image_uri);

        self.send().esdt_nft_create(
            &token_id,
            &BigUint::from(NFT_AMOUNT),
            &name,
            &BigUint::from(ROYALTIES),
            &ManagedBuffer::new(),
            &new_attributes,
            &uris,
        );
    }

    #[only_owner]
    #[endpoint(fillAttributes)]
    fn fill_attributes(&self, attributes_raw: AttributesAsMultiValue<Self::Api>) {
        let (number, background, skin, color, accessories, level) = attributes_raw.into_tuple();
        let metadata = self.build_metadata(number);

        let attributes = NftAttributes {
            background,
            skin,
            color,
            accessories,
            level,
            metadata,
        };
        self.attributes(number).set(&attributes);
    }

    #[view(getAttributForNft)]
    fn get_attribut_for_nft(&self, nft_nonce: u64, trait_index: u64) -> ManagedBuffer {
        let nft_attributes = self
            .nft_token()
            .get_token_attributes::<NftAttributes<Self::Api>>(nft_nonce);

        match trait_index {
            1 => nft_attributes.background,
            2 => nft_attributes.skin,
            3 => nft_attributes.color,
            4 => nft_attributes.accessories,
            5 => nft_attributes.metadata,
            _ => sc_panic!("Not found"),
        }
    }

    fn build_uri(&self, number: u64) -> ManagedBuffer {
        let mut uri = ManagedBuffer::new_from_bytes(IPFS_GATEWAY);
        let cid = self.image_cid().get();
        let slash = ManagedBuffer::from("/".as_bytes());
        let index_file = ManagedBuffer::new_from_bytes(number.to_string().as_bytes());
        let uri_extension = ManagedBuffer::from(".png".as_bytes());

        uri.append(&cid);
        uri.append(&slash);
        uri.append(&index_file);
        uri.append(&uri_extension);

        uri
    }

    fn build_metadata(&self, number: u64) -> ManagedBuffer {
        let metadata_key_name = ManagedBuffer::new_from_bytes(METADATA_KEY_NAME);
        let metadata_cid = self.metadata_cid().get();
        let slash = ManagedBuffer::from("/".as_bytes());
        let index_file = ManagedBuffer::new_from_bytes(number.to_string().as_bytes());
        let file_extension = ManagedBuffer::new_from_bytes(METADATA_FILE_EXTENSION);

        let mut metadata = ManagedBuffer::new();
        metadata.append(&metadata_key_name);
        metadata.append(&metadata_cid);
        metadata.append(&slash);
        metadata.append(&index_file);
        metadata.append(&file_extension);

        metadata
    }

    #[storage_mapper("attributes")]
    fn attributes(&self, number: u64) -> SingleValueMapper<NftAttributes<Self::Api>>;

    #[storage_mapper("imageCid")]
    fn image_cid(&self) -> SingleValueMapper<ManagedBuffer>;

    #[storage_mapper("metadataCid")]
    fn metadata_cid(&self) -> SingleValueMapper<ManagedBuffer>;
}
