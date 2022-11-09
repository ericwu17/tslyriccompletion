import React from "react";
import ReactDOM from "react-dom/client";
import "./index.css";
import App from "./App";
import reportWebVitals from "./reportWebVitals";

import axios from "axios";

// the "process" variable is defined during Node runtimes
// but not when running in the browser.
// This is fine, since we use npm run build to compile everything anyway.
// eslint-disable-next-line no-undef
axios.defaults.baseURL = process.env.REACT_APP_AXIOS_BASE_URL;

const root = ReactDOM.createRoot(document.getElementById("root"));
root.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
