import * as React from 'react';
import Form from 'react-bootstrap/Form'; 
import Button from 'react-bootstrap/Button'; 
import { HttpAgent, Actor } from "@dfinity/agent";
import {idlFactory} from "../../../declarations/search/search.did.js";

const Search = () => {
    const agent = new HttpAgent({ host: "https://ic0.app" });
    console.log(idlFactory);
    const searchActor = Actor.createActor(idlFactory, {
      agent,
      canisterId: "rrkah-fqaaa-aaaaa-aaaaq-cai",
    });

    const [searchText, setSearchText] = React.useState({});

    const updateState = (e) => {
        setSearchText(e.target.value);
    };

    const searchTerm = async () => {
        const terms = searchText.split(" ");
        console.log(terms);
        await searchActor.search(terms, 0, 20).then(res => {
            console.log(res)
        });
        console.log(results);
    }

     return (
        <div className="search-form">
            <Form>
                <Form.Group className="mb-3" controlId="formBasicName">
                    <Form.Control name="terms" onChange={updateState} size="lg" type="text" placeholder="Search" />
                </Form.Group> 
                <Button onClick={async () => await searchTerm()} variant="primary" type="submit">
                  DeFind Stuff :)
                </Button>              
            </Form>
        </div>
    )
};

export default Search;