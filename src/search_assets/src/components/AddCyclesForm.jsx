import * as React from 'react';
import Form from 'react-bootstrap/Form';
import Button from 'react-bootstrap/Button';

const AddCyclesForm = ({cancelDeposit, depositCycles}) => {
    const [amount, setAmount] = React.useState(0);

    const updateState = (e) => {
       setAmount(e.target.value);
    };

    return (
        <div className="website-form">
            <Form>
                <Form.Group className="mb-3" controlId="formBasicName">
                    <Form.Label>Deposit Amount</Form.Label>
                    <Form.Control name="amount" onChange={updateState} size="sm" type="number" placeholder="Amount" />
                </Form.Group>
                <Button onClick={() => depositCycles(amount)} variant="primary" type="submit">
                  Submit
                </Button>
                <Button onClick={cancelDeposit} variant="danger">Cancel</Button>
            </Form>
        </div>
    )
};

export default AddCyclesForm;