The order of parsing and range checking matters.
Each fallible operation has a different domain error.
Use map_err followed by ? to keep the success path clear.
