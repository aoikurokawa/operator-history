import React, { useEffect, useState } from "react";
import { useHistory } from "react-router-dom";
import { useDispatch, useSelector } from "react-redux";
import { useMoralisQuery } from "react-moralis";
import {
  Card,
  CardHeader,
  Avatar,
  IconButton,
  CardMedia,
  CardContent,
  Typography,
  Link,
  makeStyles,
} from "@material-ui/core";
import GavelIcon from "@material-ui/icons/Gavel";
import OpenInNewIcon from "@material-ui/icons/OpenInNew";
import { red } from "@material-ui/core/colors";
import { PulseLoader } from "react-spinners";

import { getNFTFromMoralis } from "../actions/artTokenAction";

const useStyles = makeStyles((theme) => ({
  root: {
    paddingTop: "1rem",
  },
  loader: {
    textAlign: "center",
    marginTop: "50%",
  },
  container: {
    marginTop: "5rem",
  },
  cardContainer: {
    display: "flex",
    overflow: "hidden",
    flexWrap: "wrap",
    justifyContent: "center",
  },
  cardRoot: {
    padding: "2rem",
  },
  card: {
    maxWidth: 500,
  },
  carouselItem: {
    backgroundColor: "#3F50B5",
    padding: "88px 140px",
  },
  media: {
    height: 0,
    paddingTop: "56.25%", // 16:9
  },
  expand: {
    transform: "rotate(0deg)",
    marginLeft: "auto",
    transition: theme.transitions.create("transform", {
      duration: theme.transitions.duration.shortest,
    }),
  },
  expandOpen: {
    transform: "rotate(180deg)",
  },
  avatar: {
    backgroundColor: red[500],
  },
}));

interface RootState {
  artToken: any;
  loader: any;
}

const MyPage = () => {
  const classes = useStyles();
  const history = useHistory();
  const [arrayData, setArrayData] = useState<object[] | []>([]);
  const dispatch = useDispatch();
  const { accounts, nftArray } = useSelector((state: RootState) => state.artToken);
  const { isLoading } = useSelector((state: RootState) => state.loader);

  useEffect(() => {
    
    // dispatch({
    //   type: "SHOW_LOADER",
    // });

    dispatch(getNFTFromMoralis());
    
  }, []);

  const handleModal = (objectId: string, tokenId: string) => {
    dispatch({
      type: "SHOW_MODAL",
      functionType: "MyPage",
      title: "Do you want to put up your NFT for auction?",
      objectId: objectId,
      tokenId: tokenId,
    });
  };

  const auctionDetailHandler = (auctionDetail: any) => {
    dispatch({
      type: "MOVE_AUCTIONDETAIL",
      nftDetail: auctionDetail,
    });
    history.push("./auctionDetail");
  };

  return (
    <div className={classes.root}>
      {nftArray && (
        <>
          <div className={classes.container}>
            <Typography component="h1" variant="h2" align="center">
              Your NFT
            </Typography>
            <Typography
              component="p"
              variant="inherit"
              align="right"
              style={{ paddingRight: "2rem" }}
            >
              <Link href="#">See more</Link>
            </Typography>
            <div className={classes.cardContainer}>
              {nftArray.length === 0 ?
                <div>No array</div>
                :
                <>
                {nftArray.map((d: any) => {
                  return (
                    <div
                      key={d.id}
                      className={classes.cardRoot}
                    >
                      <Card className={classes.card}>
                        {d.attributes["IsSelled"] ? (
                          <div
                            style={{
                              display: "flex",
                              justifyContent: "space-between",
                              padding: "0 1rem",
                            }}
                          >
                            <Typography component="span" variant="h4">
                              Auction Now
                            </Typography>
                            <IconButton
                              aria-label="settings"
                              onClick={() => auctionDetailHandler(d)}
                            >
                              <OpenInNewIcon />
                            </IconButton>
                          </div>
                        ) : (
                          <CardHeader
                            avatar={
                              <Avatar
                                aria-label="recipe"
                                className={classes.avatar}
                              >
                                {d.attributes["Account"]}
                              </Avatar>
                            }
                            action={
                              <IconButton
                                aria-label="settings"
                                onClick={() =>
                                  handleModal(d.id, d.attributes["TokenId"])
                                }
                              >
                                <GavelIcon />
                              </IconButton>
                            }
                            title={d.attributes["Name"]}
                          />
                        )}
                        <CardMedia
                          className={classes.media}
                          image={d.attributes["IpfsUrl"]}
                          title="Paella dish"
                        />
                        <CardContent>
                          <Typography
                            variant="body2"
                            color="textSecondary"
                            component="p"
                          >
                            IPFS Hash: <br />
                            {d.attributes["IpfsHash"]}
                          </Typography>
                          <Typography
                            variant="body2"
                            color="textSecondary"
                            component="p"
                          >
                            IPFS URL: <br />
                            <Link href={d.attributes["IpfsUrl"]} target="_blank">
                              {d.attributes["IpfsUrl"]}
                            </Link>
                          </Typography>
                          <Typography
                            variant="body2"
                            color="textSecondary"
                            component="p"
                          >
                            Token ID: {d.attributes["TokenId"]}
                          </Typography>
                        </CardContent>
                      </Card>
                    </div>
                  );
                })}
                </>
            }
              
            </div>
          </div>
        </>
      )}
    </div>
  );
};

export default MyPage;
