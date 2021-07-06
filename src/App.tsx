import React from "react";
import { Switch, Route, useLocation } from "react-router-dom";
import { useSelector } from "react-redux";

import Nav from "./components/Nav";
import Home from "./pages/Home";
import Auction from "./pages/Auction";
import AuctionDetail from "./pages/AuctionDetail";
import SpeedDials from "./components/SpeedDial";
import UploadPage from "./pages/UploadPage";
import MyPage from "./pages/MyPage";
import Loader from "./components/Loader";
import Modals from "./components/Modal";

interface RootState {
  modal: any, 
  loader: any,
}

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
        <Route path="/upload" exact>
          <UploadPage />
        </Route>
        <Route path="/mypage" exact>
          <MyPage />
        </Route>
      </Switch>
      <SpeedDials />
      <Loader />
      <Modals />
    </div>
  );
}

export default App;
