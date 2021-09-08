import WebsiteDisplay from "./WebsiteDisplay";

const ResultsDisplay = ({websites}) => {
    return (
        <div className="results">
        {
            websites.length > 0 && websites.map((site, idx) => {
                return <WebsiteDisplay key={idx} website={site} />
            })
        }
        </div>

    )
}

export default ResultsDisplay;