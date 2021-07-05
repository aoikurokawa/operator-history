import React from 'react';
import { Switch, Route, useLocation } from 'react-router-dom';

import Nav from "./components/Nav";
import Home from './pages/Home';
import Auction from './pages/Auction';
import AuctionDetail from './pages/AuctionDetail';
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
        <Route path="/auction" exact>
          <Auction />
        </Route>
        <Route path="/auctionDetail" exact>
          <AuctionDetail />
        </Route>
      </Switch>
      <SpeedDials />
    </div>
  );
}

export default App;
