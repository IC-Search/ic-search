import * as React from "react";
import NavBar from "react-bootstrap/Navbar";
import Nav from 'react-bootstrap/Nav';
import { HttpAgent } from "@dfinity/agent";
import { AuthClient } from "@dfinity/auth-client";

const Navigation = () => {
    const [authenticated, setAuthenticated] = React.useState(false);
    const [authClient, setAuthClient] = React.useState(null);
    const [principal, setPrincipal] = React.useState(null);


    const handleAuthenticated = () => {
        console.log(authClient)
        const identity = authClient.getIdentity();
        const agent = new HttpAgent({identity, host: "https://ic0.app"});
        const principal = agent.getPrincipal();
        setPrincipal(principal);
        setAuthenticated(true);
    };

    const handleLogin = () => {
        authClient.login({
            maxTimeToLive: BigInt(7 * 24 * 60 * 60 * 1e9), 
            onSuccess: () => handleAuthenticated(authClient),       
        })
    };

    React.useEffect(() => {
        (async () => {
            const authClient = await AuthClient.create();
            console.log(authClient)
            // console.log(await authClient.isAuthenticated())
            setAuthClient(authClient);
            if(await authClient.isAuthenticated()) {
                handleAuthenticated();
            }
        })();
    }, [])
 
    return (
        <div className="nav-bar"> 
            <NavBar bg="dark">
                <Nav className="ml-auto">
                    {!authenticated && 
                        <Nav.Item>
                            <Nav.Link onClick={handleLogin} className="justify-content-center">Login</Nav.Link>
                        </Nav.Item>
                    }
                </Nav>
            </NavBar>
        </div>
    )
}

export default Navigation;