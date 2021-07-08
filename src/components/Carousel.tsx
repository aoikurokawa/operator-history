import React from "react";
import { makeStyles } from "@material-ui/core";
import Carousel from "react-material-ui-carousel";
import Card from "@material-ui/core/Card";
import CardActionArea from "@material-ui/core/CardActionArea";
import CardActions from "@material-ui/core/CardActions";
import CardContent from "@material-ui/core/CardContent";
import CardMedia from "@material-ui/core/CardMedia";
import Typography from "@material-ui/core/Typography";
import { Paper, Button } from "@material-ui/core";

const useStyles = makeStyles({
  root: {
    // maxWidth: 345,
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

const Carousels = () => {
  const classes = useStyles();
  var items = [
    {
      name: "Random Name #1",
      description: "Probably the most random thing you have ever seen!",
    },
    {
      name: "Random Name #2",
      description: "Hello World!",
    },
  ];

  return (
    <Carousel
      fullHeightHover={false}
      next={() => console.log("Next")}
      prev={() => console.log("Prev")}
    >
      {items.map((item, i) => (
        <>
        <Card className={classes.root}>
          <CardActionArea>
            <CardMedia
              component="img"
              alt="Contemplative Reptile"
              height="140"
              image="/static/images/cards/contemplative-reptile.jpg"
              title="Contemplative Reptile"
            />
            <CardContent>
              <Typography gutterBottom variant="h5" component="h2">
                River
              </Typography>
              <Typography variant="body2" color="textSecondary" component="p">
                A beautifully illustrated children's book about facing the loss
                we all encounter in our lives
              </Typography>
            </CardContent>
          </CardActionArea>
          <CardActions>
            <Button size="small" color="primary">
              Share
            </Button>
            <Button size="small" color="primary">
              Learn More
            </Button>
          </CardActions>
        </Card>
        </>
      ))}
    </Carousel>
  );
};

export default Carousels;
