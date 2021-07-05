import React from 'react';
import { Switch, Route, useLocation } from 'react-router-dom';

import Nav from "./components/Nav";
import Home from './pages/Home';
import SpeedDials from './components/SpeedDial';


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
      <SpeedDials />
    </div>
  );
}

export default App;
