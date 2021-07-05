const initialState = {
    isLoading: false,
}

const loaderReducer = (state = initialState, action: any) => {
    switch(action.type) {
        case "SHOW_LOADER": 
            return {
                isLoading: true,
            };

        case "CLOSE_LOADER":
            return {
                isLoading: false, 
            };

        default: 
            return {
                ...state,
            }
    }
}

export default loaderReducer;
