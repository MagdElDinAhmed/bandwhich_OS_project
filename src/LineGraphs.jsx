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
    }, 5000);

    return () => clearInterval(interval);
  }, [selectedItem, dataType]);

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
        rates = await invoke("gcr", { connection: item, time: "Last Hour" });
      } else if (dataType === "processes") {
        rates = await invoke("gpr", { process: item, time: "Last Hour" });
      } else if (dataType === "remoteAddresses") {
        rates = await invoke("grar", {
          remoteAddress: item,
          time: "Last Hour",
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
        time: "Last Hour",
      });
    } else if (dataType === "processes") {
      rates = await invoke("gpr", { process: selectedItem, time: "Last Hour" });
    } else if (dataType === "remoteAddresses") {
      rates = await invoke("grar", {
        remoteAddress: selectedItem,
        time: "Last Hour",
      });
    }

    const chartData = [["Time", "Download Rate", "Upload Rate"]];
    rates.forEach((rate) => {
      chartData.push([new Date(rate[0]), parseInt(rate[1]), parseInt(rate[2])]);
    });
    setLineGraphData(chartData);
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
            {/* <InputLabel style={{ color: "#1a1a1a" }}>Data Type</InputLabel> */}
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
                {/* <InputLabel style={{ color: "#1a1a1a" }}>{dataType}</InputLabel> */}
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
        </div>
        {lineGraphData && (
          <div>
            <Typography sx={{ marginLeft: "120px" }} variant="h6">
              Line Graph for {selectedItem}
            </Typography>
            <Chart
              chartType="LineChart"
              data={lineGraphData}
              options={{
                title: `Download and Upload Rates for ${selectedItem}`,
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
