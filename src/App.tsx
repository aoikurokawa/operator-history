import React from 'react';
import { Switch, Route, useLocation } from 'react-router-dom';

import Nav from "./components/Nav";
import Home from './pages/Home';


function App() {
  const location = useLocation();

  return (
    <div>
      <Nav />
      <Switch location={location} key={location.pathname}>
        <Route path="/" exact>
          <Home />
        </Route>
      </Switch>
    </div>
  );
}

export default App;
