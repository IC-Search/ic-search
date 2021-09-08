import * as React from 'react';
import Button from 'react-bootstrap/Button';
import {idlFactory} from "../../../declarations/search/search.did.js";
import {search} from "../../../declarations/search";
import { Actor, HttpAgent } from "@dfinity/agent";
import { AuthClient } from "@dfinity/auth-client";
import AddWebsiteForm from '../components/AddWebsiteForm';
import AddCyclesForm from '../components/AddCyclesForm';
import WebsiteList from '../components/WebsiteList';

const Dashboard = () => {
    const [balance, setBalance] = React.useState(0);
    const [isAdding, setIsAdding] = React.useState(false);
    const [addingCycles, setAddingCycles] = React.useState(false);
    const [websites, setWebsites] = React.useState([]);
    const [agent, setAgent] = React.useState(null);
    const [searchActor, setSearchActor] = React.useState(null);
    const [authClient, setAuthClient] = React.useState(null);

    React.useEffect(() => {
        (async () => {
            const authClient = await AuthClient.create();
            setAuthClient(authClient);
            if (await authClient.isAuthenticated()) {
                handleSetup(authClient);
            }
        })();
    }, []);

    const handleSetup = async (authClient) => {
        const identity = authClient.getIdentity();
        const agent = new HttpAgent({identity, host: "https://ic0.app"});
        const searchActor = Actor.createActor(idlFactory, {
            agent,
            canisterId: "rrkah-fqaaa-aaaaa-aaaaq-cai"
        });
        const cycles =  await searchActor.get_unstaked_cycles();
        const websites = await searchActor.get_websites();
        setAgent(agent);
        setSearchActor(searchActor);
    }

    const cancelAdd = () => {
        setIsAdding(false);
    };

    const cancelDeposit = () => {
        setAddingCycles(false);
    };

    const addWebsite = (e, website) => {
        e.preventDefault();
        const result =  searchActor.set_description(website);
        console.log(result);
    };

    const depositCycles = async (amount) => {
        const result = await searchActor.deposit_cycles();
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
                        <Button onClick={() => setAddingCycles(true)} disabled={addingCycles} size="sm"className="add-website-btn" title="Add Cycles">+</Button>
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