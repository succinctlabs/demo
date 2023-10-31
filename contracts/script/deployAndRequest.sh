source ../.env
forge script SuccinctAlphaDemoScript \
    --rpc-url $RPC_5 \
    --private-key $PRIVATE_KEY \
    --broadcast \
    --sender 0xDEd0000E32f8F40414d3ab3a830f735a3553E18e \
    --verify \
    --etherscan-api-key $ETHERSCAN_API_5