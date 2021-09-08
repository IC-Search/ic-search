import * as React from "react";
import NavBar from "react-bootstrap/Navbar";
import Nav from 'react-bootstrap/Nav';
import NavDropdown from 'react-bootstrap/NavDropdown'; 
import DropdownButton from 'react-bootstrap/DropdownButton';
import Dropdown from 'react-bootstrap/Dropdown';
import { HttpAgent } from "@dfinity/agent";
import { AuthClient } from "@dfinity/auth-client";
import { Link } from 'react-router-dom';

const Navigation = () => {
    const [authenticated, setAuthenticated] = React.useState(false);
    const [authClient, setAuthClient] = React.useState(null);
    const [publicKey, setPublicKey] = React.useState("");
    const [test, setTest] = React.useState(null);
    const [principal, setPrincipal] = React.useState(null);

    const handleAuthenticated = (authClient) => {
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

    const handleLogout = () => {
        setAuthenticated(false);
        setPrincipal(null);
    };

    React.useEffect(() => {
        handleLogout();
        (async () => {
            const authClient = await AuthClient.create();
            setAuthClient(authClient);
            if(await authClient.isAuthenticated()) {
                setAuthenticated(true);
                handleAuthenticated(authClient);
            }
        })();
    }, [])
 
    return (
        <div className="nav-bar"> 
            <NavBar bg="dark">
                <Nav>
                    <NavBar.Brand>
                        <Nav.Link className="justify-content-center">
                            <Link to="/">DeFind</Link>
                        </Nav.Link>                       
                    </NavBar.Brand>
                </Nav>
                <Nav className="ml-auto">
                    {!authenticated && 
                        <Nav.Item>
                            <Nav.Link onClick={handleLogin} className="justify-content-center">Login</Nav.Link>
                        </Nav.Item>
                    }
                    {authenticated &&
                        <Nav.Item>
                           <Link to={`/dashboard/${JSON.parse(localStorage.getItem('ic-identity'))[0]}`}>My Sites</Link>
                        </Nav.Item>
                    }
                </Nav>
            </NavBar>
        </div>
    )
}

export default Navigation;