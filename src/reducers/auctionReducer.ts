const initialState = {
    contractInstance: {},
    accounts: "",
    highestBid: 0,
    auctionTimeEnd: "",
}

const auctionReducer = (state = initialState, action: any) => {
    switch (action.type) {
        case "GET_CONTRACTDATA":
            return {
                ...state,
                contractInstance: action.contractInstance,
                accounts: action.accounts,
            };

        case "GET_HIGHESTBID":
            return {
                ...state, 
                highestBid: action.highestBid,
            }

        case "GET_AUCTIONTIMEEND": 
            return {
                ...state, 
                auctionTimeEnd: action.auctionTimeEnd,
            }

        default:
            return {
                ...state
            }
    }
}

export default auctionReducer;
