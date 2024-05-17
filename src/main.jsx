import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import Tauri from "./Tauri"
import "./styles.css";

ReactDOM.createRoot(document.getElementById("root")).render(
  <React.StrictMode>
    {/* <App /> */}
    <Tauri />
  </React.StrictMode>,
);