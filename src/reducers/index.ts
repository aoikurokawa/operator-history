import { combineReducers } from "redux";

import artTokenReducer from "./artTokenReducer";    
import auctionReducer from "./auctionReducer";
import loaderReducer from "./loaderReducer";
import modalReducer from "./modalReducer";


const rootReducer = combineReducers({
    artToken: artTokenReducer,
    auction: auctionReducer, 
    loader: loaderReducer, 
    modal: modalReducer,
});

export default rootReducer;
