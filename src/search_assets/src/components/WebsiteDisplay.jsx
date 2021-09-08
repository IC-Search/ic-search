import * as React from 'react';

const WebsiteDisplay = ({website}) => {
    const goToSite = () => {
        window.location = website.link
    }

    return (
        <div className="results">
            <div align="left" onClick={goToSite} className="results_name">
                {website.name} - <span>{website.link}</span>
            </div>
            <div align="left" className="results_description">
                {website.description}
            </div>
            <hr />
        </div>

    )
}

export default WebsiteDisplay;