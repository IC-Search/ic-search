import * as React from 'react';

const WebsiteList = ({websites}) => {
    return (
        <div className="website-list">
            {websites.length > 0 && 
                websites.map((site) => {
                    return <div>Site</div>
                })
            }
            {
                websites.length === 0 && <div>No Sites Found</div>
            }
        </div>
    )
};

export default WebsiteList;