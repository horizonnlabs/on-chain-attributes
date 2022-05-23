#![no_std]
#![feature(generic_associated_types)]

extern crate alloc;
use alloc::string::ToString;

const IPFS_GATEWAY: &[u8] = "https://ipfs.io/ipfs/".as_bytes();
const METADATA_KEY_NAME: &[u8] = "metadata:".as_bytes();
const METADATA_FILE_EXTENSION: &[u8] = ".json".as_bytes();

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod token;

#[derive(ManagedVecItem, TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone, PartialEq, Debug)]
pub struct NftAttributes<M: ManagedTypeApi> {
    pub background: ManagedBuffer<M>,
    pub skin: ManagedBuffer<M>,
    pub color: ManagedBuffer<M>,
    pub accessories : ManagedBuffer<M>,
    pub metadata: ManagedBuffer<M>,
}

pub type AttributesAsMultiValue<M> = 
    MultiValue5<u64, ManagedBuffer<M>, ManagedBuffer<M>, ManagedBuffer<M>, ManagedBuffer<M>>;

#[elrond_wasm::contract]
pub trait OnChainAttributes: token::TokenModule {
    #[init]
    fn init(&self, image_cid: ManagedBuffer, metadata_cid: ManagedBuffer) 
    {
        self.image_cid().set(&image_cid);
        self.metadata_cid().set(&metadata_cid);
    }

    #[only_owner]
    #[endpoint(createWithOnChainAttributes)]
    fn create_nft_with_on_chain_attributes(&self, name: ManagedBuffer, number: u64) {
        require!(!self.nft_token_id().is_empty(), "Token is not issued");
        require!(!self.attributes(number).is_empty(), "On-chain attributes doesn't exist");

        let token = self.nft_token_id().get();
        let on_chain_attributes = self.attributes(number).get();

        let uri = self.build_uri(number);
        let mut uris = ManagedVec::new();
        uris.push(uri);

        self.send().esdt_nft_create::<NftAttributes<Self::Api>>(
            &token,
            &BigUint::from(1u64),
            &name,
            &BigUint::from(100u64),
            &ManagedBuffer::new(),
            &on_chain_attributes,
            &uris,
        );
    }

    #[view(getAttributForNft)]
    fn get_attribut_for_nft(&self, nonce: u64, trait_index: u64) -> ManagedBuffer {
        let token = self.nft_token_id().get();
        let nft_data = self.blockchain().get_esdt_token_data(
            &self.blockchain().get_sc_address(),
            &token,
            nonce
        );

        let nft_attributes = nft_data
            .decode_attributes::<NftAttributes<Self::Api>>();

        match trait_index {
            1 => nft_attributes.background, // return ocean
            2 => nft_attributes.skin, // return boss
            3 => nft_attributes.color, // return blue
            4 => nft_attributes.accessories, // return gun
            5 => nft_attributes.metadata, // return metadata:<cid>/1.json
            _ => sc_panic!("Not found")
        }
    }

    #[only_owner]
    #[endpoint(fillAttributes)]
    fn fill_attributes_endpoint(
        &self,
        attributes: MultiValueEncoded<AttributesAsMultiValue<Self::Api>>,
    ) {
        for attribut in attributes.into_iter() {
            let (number, background, skin, color, accessories) = attribut.into_tuple();
            let metadata = self.build_metadata(number);

            let attributes = NftAttributes {
                background,
                skin,
                color,
                accessories,
                metadata
            };
            self.attributes(number).set(&attributes);
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