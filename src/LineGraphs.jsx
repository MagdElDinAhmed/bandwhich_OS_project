import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api";
import {
  Typography,
  Select,
  MenuItem,
  FormControl,
  InputLabel,
} from "@mui/material";
import "./Tauri.css";
import { Chart } from "react-google-charts";
import CustomNav from "./Nav";

export default function LineGraphs() {
  const [dataType, setDataType] = useState(""); // "connections", "processes", or "remoteAddresses"
  const [dataList, setDataList] = useState([]);
  const [selectedItem, setSelectedItem] = useState("");
  const [lineGraphData, setLineGraphData] = useState(null);
  const [refreshRate, setRefreshRate] = useState(5000); // Default refresh rate is 5000ms (5 seconds)
  const [selectedTimeRange, setSelectedTimeRange] = useState("Last Hour");

  useEffect(() => {
    if (dataType) {
      fetchDataList();
    }
  }, [dataType]);

  useEffect(() => {
    if (selectedItem) {
      fetchLineGraphData();
    }
  }, [selectedItem]);

  useEffect(() => {
    const interval = setInterval(() => {
      if (selectedItem) {
        fetchLineGraphData();
      }
    }, refreshRate);

    return () => clearInterval(interval);
  }, [selectedItem, dataType, refreshRate, selectedTimeRange]);

  const fetchDataList = async () => {
    let list;
    if (dataType === "connections") {
      list = await invoke("gcl");
    } else if (dataType === "processes") {
      list = await invoke("gpl");
    } else if (dataType === "remoteAddresses") {
      list = await invoke("gral");
    }
    const filteredList = [];
    for (let item of list) {
      let rates;
      if (dataType === "connections") {
        rates = await invoke("gcr", {
          connection: item,
          time: selectedTimeRange,
        });
      } else if (dataType === "processes") {
        rates = await invoke("gpr", { process: item, time: selectedTimeRange });
      } else if (dataType === "remoteAddresses") {
        rates = await invoke("grar", {
          remoteAddress: item,
          time: selectedTimeRange,
        });
      }
      if (rates.length > 0 && rates[0][0] !== "No data available") {
        filteredList.push(item);
      }
    }
    setDataList(filteredList);
  };

  const fetchLineGraphData = async () => {
    let rates;
    if (dataType === "connections") {
      rates = await invoke("gcr", {
        connection: selectedItem,
        time: selectedTimeRange,
      });
    } else if (dataType === "processes") {
      rates = await invoke("gpr", {
        process: selectedItem,
        time: selectedTimeRange,
      });
    } else if (dataType === "remoteAddresses") {
      rates = await invoke("grar", {
        remoteAddress: selectedItem,
        time: selectedTimeRange,
      });
    }

    const chartData = [["Time", "Upload Rate", "Download Rate"]];
    rates.forEach((rate) => {
      chartData.push([new Date(rate[0]), parseInt(rate[1]), parseInt(rate[2])]);
    });
    setLineGraphData(chartData);
  };

  const handleRefreshRateChange = (event) => {
    const { value } = event.target;
    setRefreshRate(parseInt(value));
  };

  const handleTimeRangeChange = (event) => {
    setSelectedTimeRange(event.target.value);
  };

  return (
    <div className="darkBackground">
      <CustomNav />
      <div
        style={{
          padding: "20px",
          color: "white",
          display: "flex",
          justifyContent: "center",
          flexDirection: "column",
        }}
      >
        <div
          style={{
            width: "800px",
            margin: "auto",
          }}
        >
          <Typography variant="h6">Select Data Type</Typography>
          <FormControl
            variant="filled"
            className="basicDropDown"
            style={{ width: "100%", marginBottom: 20, height: 50 }}
          >
            <Select
              value={dataType}
              onChange={(e) => {
                setDataType(e.target.value);
                setSelectedItem("");
                setDataList([]);
                setLineGraphData(null);
              }}
              style={{ color: "#1a1a1a", transform: "translateY(-6px)" }}
            >
              <MenuItem value="connections">Connections</MenuItem>
              <MenuItem value="processes">Processes</MenuItem>
              <MenuItem value="remoteAddresses">Remote Addresses</MenuItem>
            </Select>
          </FormControl>

          {dataType && (
            <>
              <Typography variant="h6">Select {dataType}</Typography>
              <FormControl
                variant="filled"
                className="basicDropDown"
                style={{ width: "100%", marginBottom: 20, height: 50 }}
              >
                <Select
                  value={selectedItem}
                  onChange={(e) => setSelectedItem(e.target.value)}
                  style={{ color: "#1a1a1a", transform: "translateY(-6px)" }}
                >
                  {dataList.map((item) => (
                    <MenuItem key={item} value={item}>
                      {item}
                    </MenuItem>
                  ))}
                </Select>
              </FormControl>
            </>
          )}

          <Typography variant="h6">Select Time Range</Typography>
          <FormControl
            variant="filled"
            className="basicDropDown"
            style={{ width: "100%", marginBottom: 20, height: 50 }}
          >
            <Select
              value={selectedTimeRange}
              onChange={handleTimeRangeChange}
              style={{ color: "#1a1a1a", transform: "translateY(-6px)" }}
            >
              <MenuItem value="Last Hour">Last Hour</MenuItem>
              <MenuItem value="Last Day">Last Day</MenuItem>
              <MenuItem value="Last Week">Last Week</MenuItem>
            </Select>
          </FormControl>
        </div>
        <div
          style={{
            marginTop: "20px",
            marginBottom: "20px",
            display: "flex",
            justifyContent: "center",
            alignItems: "center",
          }}
        >
          <Typography variant="h6">Refresh Rate (milliseconds):</Typography>
          <input
            type="number"
            value={refreshRate}
            onChange={handleRefreshRateChange}
            style={{ marginLeft: "10px" }}
          />
        </div>
        {lineGraphData && (
          <div style={{ marginBottom: "100px" }}>
            <Typography sx={{ marginLeft: "120px" }} variant="h6">
              Line Graph for {selectedItem}
            </Typography>
            <Chart
              chartType="LineChart"
              data={lineGraphData}
              options={{
                title: `Upload and Download Rates for ${selectedItem}`,
                titleTextStyle: { color: "white" },
                hAxis: {
                  title: "Time",
                  titleTextStyle: { color: "white" },
                  textStyle: { color: "#EAEAEA" },
                  gridlines: { color: "#4a4a4a" },
                },
                vAxis: {
                  title: "Rate",
                  minValue: 0,
                  titleTextStyle: { color: "white" },
                  textStyle: { color: "#EAEAEA" },
                  gridlines: { color: "#4a4a4a" },
                },
                legend: { position: "bottom", textStyle: { color: "white" } },
                backgroundColor: "#1a1a1a",
                colors: ["#CA1CEC", "#FFDD44"],
              }}
              width="100%"
              height="400px"
            />
          </div>
        )}
      </div>
    </div>
  );
}
