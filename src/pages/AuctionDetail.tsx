import React from 'react';

import DisplayImage from "../components/DisplayImage";
import DisplayPrice from "../components/DisplayPrice";


const AuctionDetail = () => {

    return (
        <div style={{ height: '60.4rem', padding: '1rem 5rem' }}>
            <div style={{ display: 'flex' }}>
                <DisplayImage />
                <DisplayPrice />
            </div>
        </div>
    );
}

export default AuctionDetail;
