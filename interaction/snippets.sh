PROXY="https://devnet-gateway.elrond.com"
CHAIN="D"
OWNER="../../wallet-owner.pem"
CONTRACT="output/on-chain-attributes.wasm"

IMAGE_CID=str:QmaS44fS6KJEarBaUHma8f1Fqore1bLYY8DEvPV6atAnyy
METADATA_CID=str:QmdWJ6NUZwzHUx1ZR3goHVrb8aptmVdYBdwVC1WfBZSdTR

deploy() {
    erdpy --verbose contract deploy --bytecode="$CONTRACT" --recall-nonce \
        --pem=$OWNER \
        --gas-limit=599000000 \
        --proxy=$PROXY --chain=$CHAIN \
        --metadata-payable \
        --arguments $IMAGE_CID $METADATA_CID \
        --outfile="deploy-devnet.interaction.json" --send || return

    TRANSACTION=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['contractAddress']")

    erdpy data store --key=address-devnet --value=${ADDRESS}
    erdpy data store --key=deployTransaction-devnet --value=${TRANSACTION}

    echo "Smart contract address: ${ADDRESS}"
}

upgrade() {
    erdpy --verbose contract upgrade ${ADDRESS} --bytecode="$CONTRACT" --recall-nonce \
    --pem=${OWNER} \
    --gas-limit=599000000 \
    --proxy=${PROXY} --chain=${CHAIN} \
    --metadata-payable \
    --arguments $IMAGE_CID $METADATA_CID \
    --send --outfile="deploy-devnet.interaction.json" || return

    echo ""
    echo "Smart contract upgraded address: ${ADDRESS}"
}

issueToken() {
    token_name=str:NFT
    token_ticker=str:NFT

    echo "issue token..."

    erdpy --verbose contract call ${ADDRESS} --recall-nonce \
        --pem=$OWNER \
        --gas-limit=599000000 \
        --proxy=$PROXY --chain=$CHAIN \
        --value=50000000000000000 \
        --function="issueToken" \
        --arguments $token_name $token_ticker  \
        --send || return
}

setLocalRoles() {
    echo "set local roles..."

    erdpy --verbose contract call ${ADDRESS} --recall-nonce \
        --pem=$OWNER \
        --gas-limit=599000000 \
        --proxy=$PROXY --chain=$CHAIN \
        --function="setLocalRoles" \
        --send || return
}

fillAttributes() {
    number_1=0x01
    background_1=str:ocean
    skin_1=str:boss
    color_1=str:blue
    accessories_1=str:gun
    level_1=1

    erdpy --verbose contract call ${ADDRESS} --recall-nonce \
        --pem=$OWNER \
        --gas-limit=599000000 \
        --proxy=$PROXY --chain=$CHAIN \
        --function="fillAttributes" \
        --arguments $number_1 $background_1 $skin_1 $color_1 $accessories_1 $level_1 \
        --send || return
}

createNftWithAttributesFromStorage() {
    name=str:NftOne
    number=1

    erdpy --verbose contract call ${ADDRESS} --recall-nonce \
        --pem=$OWNER \
        --gas-limit=599000000 \
        --proxy=$PROXY --chain=$CHAIN \
        --function="createNftWithAttributesFromStorage" \
        --arguments $name $number \
        --send || return
}

createNft() {
    name=str:NftOne

    number=1
    background=str:Ocean
    skin=str:Boss
    color=str:Blue
    accessories=str:Gun
    level=1

    erdpy --verbose contract call ${ADDRESS} --recall-nonce \
        --pem=$OWNER \
        --gas-limit=599000000 \
        --proxy=$PROXY --chain=$CHAIN \
        --function="createNft" \
        --arguments $name $number $background $skin $color $accessories $level \
        --send || return
}

updateAttributes() {
    nft_nonce=1

    erdpy --verbose contract call ${ADDRESS} --recall-nonce \
        --pem=$OWNER \
        --gas-limit=599000000 \
        --proxy=$PROXY --chain=$CHAIN \
        --function="updateAttributes" \
        --arguments $nft_nonce \
        --send || return
}

mintNftWithNewUriAndAttributes() {
    nft_nonce=1
    name=str:SuperNft

    background=str:Dragon
    skin=str:Voyager
    color=str:Red
    accessories=str:Axe

    #new_image_uri=https://ipfs.io/ipfs/<cid>
    #new_metadata=str:metadata:<cid>

    erdpy --verbose contract call ${ADDRESS} --recall-nonce \
        --pem=$OWNER \
        --gas-limit=599000000 \
        --proxy=$PROXY --chain=$CHAIN \
        --function="mintNftWithNewUriAndAttributes" \
        --arguments $nft_nonce $name $background $skin $color $accessories $new_image_uri $new_metadata \
        --send || return
}

getAttributForNft() {
    NUMBER=1
    TRAIT_INDEX=1
    erdpy --verbose contract query ${ADDRESS} --function="getAttributForNft" --arguments $NUMBER $TRAIT_INDEX --proxy=${PROXY} 
}