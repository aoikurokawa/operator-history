import React from "react";
import Typography from "@material-ui/core/Typography";
import Grid from "@material-ui/core/Grid";
import TextField from "@material-ui/core/TextField";
import FormControlLabel from "@material-ui/core/FormControlLabel";
import Checkbox from "@material-ui/core/Checkbox";
import TextareaAutosize from "@material-ui/core/TextareaAutosize";
import { makeStyles, InputLabel, Select, MenuItem } from "@material-ui/core";

const useStyles = makeStyles({
  textArea: {
    width: "100%",
    height: "10rem",
  },
});

const Description = () => {
  const classes = useStyles();
  const [age, setAge] = React.useState<string | number>("");
  const [open, setOpen] = React.useState(false);

  const handleChange = (event: React.ChangeEvent<{ value: unknown }>) => {
    setAge(event.target.value as number);
  };

  const handleClose = () => {
    setOpen(false);
  };

  const handleOpen = () => {
    setOpen(true);
  };

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
        <Grid item xs={12}>
          <InputLabel id="demo-controlled-open-select-label">
            Category
          </InputLabel>
          <Select
            labelId="demo-controlled-open-select-label"
            id="demo-controlled-open-select"
            open={open}
            onClose={handleClose}
            onOpen={handleOpen}
            value={age}
            onChange={handleChange}
            style={{ display: "flex" }}
          >
            <MenuItem value="">
              <em>None</em>
            </MenuItem>
            <MenuItem value={10}>Art</MenuItem>
            <MenuItem value={20}>Music</MenuItem>
            <MenuItem value={30}>Performance</MenuItem>
          </Select>
        </Grid>
      </Grid>
    </React.Fragment>
  );
};

export default Description;
