import * as React from 'react';
import Form from 'react-bootstrap/Form';
import Button from 'react-bootstrap/Button';

const AddWebsiteForm = ({cancelAdd, addWebsite}) => {
    const [state, setState] = React.useState({});

    const updateState = (e) => {
        setState({...state, [e.target.name]: e.target.value});
    }

    return (
        <div className="website-form">
            <Form>
                <Form.Group className="mb-3" controlId="formBasicName">
                    <Form.Label>Website Name</Form.Label>
                    <Form.Control name="name" onChange={updateState} size="sm" type="text" placeholder="Website Name" />
                </Form.Group>
                <Form.Group className="mb-3" controlId="formBasicPassword">
                    <Form.Label>URL</Form.Label>
                    <Form.Control name="link" onChange={updateState} size="sm" type="text" placeholder="Website Link" />
                </Form.Group>
                <Form.Group className="mb-3" controlId="formBasicPassword">
                    <Form.Label>Description</Form.Label>
                    <Form.Control name="description" onChange={updateState} size="sm" as="textarea" placeholder="Short Description" />
                </Form.Group>
                <Button onClick={() => addWebsite(state)} variant="primary" type="submit">
                  Submit
                </Button>
                <Button onClick={cancelAdd} variant="danger">Cancel</Button>
            </Form>
        </div>
    )
};

export default AddWebsiteForm;