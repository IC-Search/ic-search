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

    const handleAuthenticated = () => {
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
            const item = localStorage.getItem('ic-identity');
            console.log(JSON.parse(item)[0]);
            if(await authClient.isAuthenticated()) {
                setAuthenticated(true);
                handleAuthenticated();
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
                <Nav className="">
                    {!authenticated && 
                        <Nav.Item>
                            <Nav.Link onClick={handleLogin} className="justify-content-center">Login</Nav.Link>
                        </Nav.Item>
                    }
                    {authenticated &&
                        <DropdownButton title="=" drop="down">
                            <Dropdown.Item className="justify-content-center">
                                <Link to={`/dashboard/${JSON.parse(localStorage.getItem('ic-identity'))[0]}`}>My Sites</Link>
                            </Dropdown.Item>
                            <Dropdown.Divider />
                            <Dropdown.Item>
                                Balance: 0.00 cycles
                            </Dropdown.Item>
                        </DropdownButton>
                    }
                </Nav>
            </NavBar>
        </div>
    )
}

export default Navigation;