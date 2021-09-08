import * as React from 'react';
import Form from 'react-bootstrap/Form'; 
import Button from 'react-bootstrap/Button'; 
import {search} from "../../../declarations/search";
import WebsiteDisplay from './WebsiteDisplay';
const Search = () => {
    const [searchText, setSearchText] = React.useState({});
    const [websites, setWebsites] = React.useState([]);
    const [didSearch, setDidSearch] = React.useState(false);

    const updateState = (e) => {
        setSearchText(e.target.value);
    };

    const searchTerm = async (e) => {
        e.preventDefault();
        setDidSearch(false);
        const terms = searchText.split(" ");
        const results = await search.search(terms, 0, 10);
        setDidSearch(true);
        setWebsites(results);
    }

     return (
        <div className="search-form">
            <div className="search-form"> 
                <Form>
                    <Form.Group className="mb-3" controlId="formBasicName">
                        <Form.Control name="terms" onChange={updateState} size={didSearch && websites.length > 0 ? "sm" : "lg"} type="text" placeholder="Search" />
                    </Form.Group> 
                    <Button size={didSearch && websites.length > 0 ? "sm" : "lg"} onClick={async (e) => await searchTerm(e)} variant="primary" type="submit">
                      DeFind Stuff :)
                    </Button>              
                </Form> 
                {
                    (didSearch && websites.length === 0) && <div>No websites found...</div>
                }
            </div>          
            {
                websites.length > 0 && websites.map((site, idx) => {
                    return <WebsiteDisplay key={idx} website={site} />
                })

            }
        </div>
    )
};

export default Search;