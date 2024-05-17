import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import Tauri from "./Tauri";
import "./styles.css";
import PieCharts from "./PieCharts";

ReactDOM.createRoot(document.getElementById("root")).render(
  <React.StrictMode>
    {/* <App /> */}
    {/* <Tauri /> */}
    <PieCharts />
  </React.StrictMode>
);
