import * as React from 'react';
import Button from 'react-bootstrap/Button';
import { search } from "../../../declarations/search";
import AddWebsiteForm from '../components/AddWebsiteForm';
import WebsiteList from '../components/WebsiteList';

const Dashboard = () => {
    const [balance, setBalance] = React.useState(0);
    const [isAdding, setIsAdding] = React.useState(false);
    const [websites, setWebsites] = React.useState([]);

    React.useEffect(() => {
        // const cycles =  search.get__unstaked_cycles();
        // console.log(search);
    });

    React.useEffect(() => {
        const websites = search.get_websites();
        console.log(websites);
    })

    const cancelAdd = () => {
        setIsAdding(false);
    };

    const addWebsite = async (website) => {
        const result = await search.set_description(website);
        console.log("Result" + result);
    };

    return (
        <div className="dashboard">
            <h2>Control Center</h2>
            <div className="dashboard__main">
                <div className="dasboard-main__sites dash-card">
                    <div className="dash-card">
                        <h4>My Sites</h4> 
                    </div>
                    <div className="dash-card add-website-btn-container">
                        <Button onClick={() => setIsAdding(true)} disabled={isAdding} size="sm"className="add-website-btn" title="Add Site">+</Button>
                    </div>
                    <div>
                        {isAdding && <AddWebsiteForm addWebsite={addWebsite} cancelAdd={cancelAdd} />}
                        {!isAdding && <WebsiteList websites={websites} />}
                    </div>
                </div>
                <div className="dasboard-main__balance dash-card">
                    <h4>Balance</h4>
                    <span>{balance}</span>
                </div>
            </div>
        </div>
    )
};

export default Dashboard;