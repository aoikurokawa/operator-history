import "date-fns";
import React, { useState } from "react";
import {
  Grid,
  Typography,
  TextField,
  FormControlLabel,
  Checkbox,
  RadioGroup,
  Radio,
  makeStyles,
} from "@material-ui/core";
import DateFnsUtils from "@date-io/date-fns";
import {
  MuiPickersUtilsProvider,
  KeyboardTimePicker,
  KeyboardDatePicker,
} from "@material-ui/pickers";

const useStyles = makeStyles({
  date: {
    width: "100%",
  },
});

const Target = () => {
  const classes = useStyles();

  const [selectedDate, setSelectedDate] = useState<Date | null>(new Date());

  const handleDateChange = (date: Date | null) => {
    setSelectedDate(date);
  };
  return (
    <React.Fragment>
      <MuiPickersUtilsProvider utils={DateFnsUtils}>
        <Typography variant="h6" gutterBottom>
          Let's set the required target amount
        </Typography>
        <Grid container spacing={3}>
          <Grid item xs={12}>
            <TextField
              required
              id="targetAmount"
              name="targetAmount"
              label="Target Amount"
              type="number"
              fullWidth
              autoComplete="target-amount"
            />
          </Grid>
          <Grid item xs={12}>
            <KeyboardDatePicker
              margin="normal"
              id="date-picker-dialog"
              label="End Date"
              format="MM/dd/yyyy"
              value={selectedDate}
              onChange={handleDateChange}
              KeyboardButtonProps={{
                "aria-label": "change date",
              }}
              className={classes.date}
            />
          </Grid>
        </Grid>
      </MuiPickersUtilsProvider>
    </React.Fragment>
  );
};

export default Target;
