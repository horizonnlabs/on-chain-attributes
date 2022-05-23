PROXY="https://devnet-gateway.elrond.com"
CHAIN="D"
OWNER="../wallet-owner.pem"
CONTRACT="output/on-chain-attributes.wasm"

IMAGE_CID="0x$(echo -n 'QmaS44fS6KJEarBaUHma8f1Fqore1bLYY8DEvPV6atAnyy' | xxd -p -u | tr -d '\n')"
METADATA_CID="0x$(echo -n 'QmP9KNxdDzseRa9wmfnhazAnavwMT6f7HqpnFzJ9uXRxRg' | xxd -p -u | tr -d '\n')"

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
    token_name="0x$(echo -n 'NFT' | xxd -p -u | tr -d '\n')"
    token_ticker="0x$(echo -n 'NFT' | xxd -p -u | tr -d '\n')"

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
    NUMBER_1=0x01
    VALUE_ONE_1="0x$(echo -n 'ocean' | xxd -p -u | tr -d '\n')"
    VALUE_TWO_1="0x$(echo -n 'boss' | xxd -p -u | tr -d '\n')"
    VALUE_THREE_1="0x$(echo -n 'blue' | xxd -p -u | tr -d '\n')"
    VALUE_FOUR_1="0x$(echo -n 'gun' | xxd -p -u | tr -d '\n')"
    NUMBER_2=0x02
    VALUE_ONE_2="0x$(echo -n 'desert' | xxd -p -u | tr -d '\n')"
    VALUE_TWO_2="0x$(echo -n 'farmer' | xxd -p -u | tr -d '\n')"
    VALUE_THREE_2="0x$(echo -n 'yellow' | xxd -p -u | tr -d '\n')"
    VALUE_FOUR_2="0x$(echo -n 'none' | xxd -p -u | tr -d '\n')"

    erdpy --verbose contract call ${ADDRESS} --recall-nonce \
        --pem=$OWNER \
        --gas-limit=599000000 \
        --proxy=$PROXY --chain=$CHAIN \
        --function="fillAttributes" \
        --arguments $NUMBER_1 $VALUE_ONE_1 $VALUE_TWO_1 $VALUE_THREE_1 $VALUE_FOUR_1 $NUMBER_2 $VALUE_ONE_2 $VALUE_TWO_2 $VALUE_THREE_2 $VALUE_FOUR_2 \
        --send || return
}

createWithOnChainAttributes() {
    NAME="0x$(echo -n 'NftOne' | xxd -p -u | tr -d '\n')"
    NUMBER=0x01

    erdpy --verbose contract call ${ADDRESS} --recall-nonce \
        --pem=$OWNER \
        --gas-limit=599000000 \
        --proxy=$PROXY --chain=$CHAIN \
        --function="createWithOnChainAttributes" \
        --arguments $NAME $NUMBER \
        --send || return
}

getAttributForNft() {
    NUMBER=0x01
    TRAIT_INDEX=0x01
    erdpy --verbose contract query ${ADDRESS} --function="getAttributForNft" --arguments $NUMBER $TRAIT_INDEX --proxy=${PROXY} 
}