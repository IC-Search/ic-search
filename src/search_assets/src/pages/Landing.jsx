import * as React from "react";
import { render } from "react-dom";
import Search from "../components/Search";

const LandingPage = () => {
    return (
        <div className="landing">
            <h2 className="landing__title">DeFind</h2>
            <div>
                <Search />
            </div>
        </div>
    )
};

export default LandingPage;