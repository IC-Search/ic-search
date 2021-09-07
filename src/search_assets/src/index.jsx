import * as React from "react";
import { render } from "react-dom";
import Navigation from "./components/Navigation";
import { search } from "../../declarations/search";

const App = () => {

  return (
    <div>
      <Navigation />
      <div className="landing-main-content">
        <h2>IC Search</h2>
      </div>
    </div>
  );
};

render(<App />, document.getElementById("app"));


