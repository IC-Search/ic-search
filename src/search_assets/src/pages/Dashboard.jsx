import * as React from 'react';
import Button from 'react-bootstrap/Button';
import { search } from "../../../declarations/search";
import AddWebsiteForm from '../components/AddWebsiteForm';
import AddCyclesForm from '../components/AddCyclesForm';
import WebsiteList from '../components/WebsiteList';

const Dashboard = () => {
    const [balance, setBalance] = React.useState(0);
    const [isAdding, setIsAdding] = React.useState(false);
    const [addingCycles, setAddingCycles] = React.useState(false);
    const [websites, setWebsites] = React.useState([]);

    React.useEffect(() => {
        const cycles =  search.get_unstaked_cycles();
    });

    React.useEffect(() => {
        const websites = search.get_websites();
    })

    const cancelAdd = () => {
        setIsAdding(false);
    };

    const cancelDeposit = () => {
        setAddingCycles(false);
    };

    const addWebsite = async (website) => {
        const result = await search.set_description(website);
    };

    const depositCycles = async (amount) => {
        const result = await search.deposit_cycles();
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
                    <div className="dash-card">
                        <h4>Balance</h4>
                    </div>
                    <div className="dash-card add-website-btn-container">
                        <Button onClick={() => setAddingCycles(true)} disabled={addingCycles} size="sm"className="add-website-btn" title="Add Site">+</Button>
                    </div>
                    <div>
                        {addingCycles && <AddCyclesForm depositCycles={depositCycles} cancelDeposit={cancelDeposit} />}
                        {!addingCycles && <span>{balance}</span>}
                    </div>
                </div>
            </div>
        </div>
    )
};

export default Dashboard;