import React from "react";
import { makeStyles } from "@material-ui/core/styles";
import Button from "@material-ui/core/Button";

import Carousels from "../components/Carousel";
import { Container } from "@material-ui/core";

const useStyles = makeStyles({
  root: {
    maxWidth: 345,
  },
  carouselContainer: {
    // width: "10rem",
    padding: "2rem",
    height: "20rem",
  },
  buttonContainer: {
    display: "flex", 
    justifyContent: "space-evenly",
  }
});

const CrowdfundingPage = () => {
  const classes = useStyles();

  return (
    <>
      <Container className={classes.carouselContainer}>
        <Carousels />
      </Container>
      <Container className={classes.buttonContainer}>
          <Button variant="contained" color="primary" disableElevation>
            Create Project
          </Button>
          <Button variant="contained" color="primary" disableElevation>
            Explore Project
          </Button>
        </Container>
    </>
  );
};

export default CrowdfundingPage;
