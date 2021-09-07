import * as React from 'react';
import Button from 'react-bootstrap/Button';
import { search } from "../../declarations/search";

const Dashboard = () => {
    const [balance, setBalance] = React.useState(0);
    
    return (
        <div className="dashboard">
            <h2>Control Center</h2>
            <div className="dasboard__main">
                <div className="dasboard-main__sites">
                    <h4>My Sites</h4> <Button>+</Button>
                </div>
            </div>
        </div>
    )
};

export default Dashboard;