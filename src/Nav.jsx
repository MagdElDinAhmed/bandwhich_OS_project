import React, { useState } from "react";
import { Link } from "react-router-dom";
import "./Tauri.css";
import { Typography } from "@mui/material";
import { useNavigate } from "react-router-dom";

export default function CustomNav() {
  const navigate = useNavigate();

  return (
    <div
      style={{
        backgroundColor: "#1a1a1a",
        borderBottom: "1px solid white",
        marginBottom: "30px",
      }}
    >
      <div
        style={{
          display: "flex",
          flexDirection: "column",
        }}
      >
        <div style={{ display: "flex", flexDirection: "row" }}>
          <Typography
            sx={{
              fontFamily: "Biotic",
              color: "#87CEEB",
              margin: "10px 0px 0px 20px",
              fontSize: "40px",
              backgroundColor: "#011936",
            }}
          >
            Bandwhich
          </Typography>
        </div>
        <div
          style={{
            display: "flex",
            flexDirection: "row",
            justifyContent: "center",
            marginBottom: "20px",
          }}
        >
          <button className="navbarButton" onClick={() => navigate(`/`)}>
            Home
          </button>
          <button
            className="navbarButton"
            onClick={() => navigate(`/PieCharts`)}
          >
            Pie Charts
          </button>
          <button
            className="navbarButton"
            onClick={() => navigate(`/LineGraphs`)}
          >
            Line Graphs
          </button>
        </div>
      </div>
    </div>
  );
}
