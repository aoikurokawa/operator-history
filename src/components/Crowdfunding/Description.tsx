import React from "react";
import Typography from "@material-ui/core/Typography";
import Grid from "@material-ui/core/Grid";
import TextField from "@material-ui/core/TextField";
import FormControlLabel from "@material-ui/core/FormControlLabel";
import Checkbox from "@material-ui/core/Checkbox";
import TextareaAutosize from '@material-ui/core/TextareaAutosize';
import { makeStyles } from "@material-ui/core";

const useStyles = makeStyles({
  textArea: {
    width: "100%",
    height: "10rem",
  }
})

const Description = () => {
  const classes = useStyles();

  return (
    <React.Fragment>
      <Typography variant="h6" gutterBottom>
        Tell your project
      </Typography>
      <Grid container spacing={3}>
        <Grid item xs={12}>
          <TextField
            required
            id="title"
            label="Title"
            fullWidth
            autoComplete="cc-title"
            placeholder="Tell your original title"
          />
        </Grid>
        <Grid item xs={12}>
          <TextareaAutosize
            aria-label="maximum height"
            placeholder="Conclude your project in 3 lines"
            className={classes.textArea}
            rows={10}
          />
        </Grid>
        <Grid item xs={12} md={6}>
          <TextField
            required
            id="expDate"
            label="Expiry date"
            fullWidth
            autoComplete="cc-exp"
          />
        </Grid>
        <Grid item xs={12} md={6}>
          <TextField
            required
            id="cvv"
            label="CVV"
            helperText="Last three digits on signature strip"
            fullWidth
            autoComplete="cc-csc"
          />
        </Grid>
        <Grid item xs={12}>
          <FormControlLabel
            control={<Checkbox color="secondary" name="saveCard" value="yes" />}
            label="Remember credit card details for next time"
          />
        </Grid>
      </Grid>
    </React.Fragment>
  );
};

export default Description;
