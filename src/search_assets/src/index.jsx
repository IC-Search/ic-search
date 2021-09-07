import * as React from "react";
import { render } from "react-dom";
import Navigation from "./components/Navigation";
import { Route, Switch, HashRouter as Router } from 'react-router-dom';
import routes from './config/routes';
import { search } from "../../declarations/search";

const App = () => {

  return (
    <Router>
      <Navigation />
      <React.Suspense fallback={<div>Loading...</div>}></React.Suspense>
      <div className="main-content">
          <Switch>
            {routes.map((route, idx) => {
              return <Route key={idx} {...route} />
            })}
            <Route path="/*">
                <div>Error loading page</div>
            </Route>
          </Switch>
      </div>
    </Router>
  );
};

render(<App />, document.getElementById("app"));


