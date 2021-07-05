interface State{
    isModalVisible: boolean, 
    functionType: string, 
    title: string, 
    objectId: string, 
    tokenId: string,
}

const initialState: State = {
    isModalVisible: false,
    functionType: "",
    title: "",
    objectId: "",
    tokenId: "",
}

const modalReducer = (state = initialState, action: any) => {
    switch(action.type) {
        case 'SHOW_MODAL': 
            return {
                isModalVisible: true,
                title: action.title,
                functionType: action.functionType,
                objectId: action.objectId,
                tokenId: action.tokenId,
            }

        case 'CLOSE_MODAL':
            return {
                isModalVisible: false, 
                title: "", 
                functionType: "",
                objectId: "",
                tokenId: "",
            }

        default: 
            return {
                ...state
            }    
    }
}

export default modalReducer;