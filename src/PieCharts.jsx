import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api";
import { Typography } from "@mui/material";
import "./Tauri.css";
import { Chart } from "react-google-charts";
import CustomNav from "./Nav";

export default function PieCharts() {
  const [pieChartConnections, setPieChartConnections] = useState();
  const [pieChartProcesses, setPieChartProcesses] = useState();
  const [pieChartRemoteAddresses, setPieChartRemoteAddresses] = useState();
  const [pieChartConnectionsUpload, setPieChartConnectionsUpload] = useState();
  const [pieChartProcessesUpload, setPieChartProcessesUpload] = useState();
  const [pieChartRAUpload, setPieChartRAUpload] = useState();
  const [refreshRate, setRefreshRate] = useState(5000);

  async function fetchData(dataList, invokeCommand, type) {
    const data = [];
    if (type == "connections") {
      for (const item of dataList) {
        const rates = await invoke(invokeCommand, {
          connection: item,
          time: "Last Hour",
        });
        if (rates[rates.length - 1][0] == "No data available") {
          continue;
        }
        data.push({
          name: item,
          upload: Math.abs(rates[rates.length - 1][1] - rates[0][1]),
          download: Math.abs(rates[rates.length - 1][2] - rates[0][2]),
          date: new Date(rates[rates.length - 1][0]),
        });
      }
    } else if (type == "processes") {
      for (const item of dataList) {
        const rates = await invoke(invokeCommand, {
          process: item,
          time: "Last Hour",
        });
        if (rates[rates.length - 1][0] == "No data available") {
          continue;
        }
        // console.log("here:", rates);
        data.push({
          name: item,
          upload: Math.abs(rates[rates.length - 1][1] - rates[0][1]),
          download: Math.abs(rates[rates.length - 1][2] - rates[0][2]),
          date: new Date(rates[rates.length - 1][0]),
        });
      }
    } else if (type == "remoteAddresses") {
      for (const item of dataList) {
        const rates = await invoke(invokeCommand, {
          remoteAddress: item,
          time: "Last Hour",
        });
        if (rates[rates.length - 1][0] == "No data available") {
          continue;
        }
        data.push({
          name: item,
          upload: Math.abs(rates[rates.length - 1][1] - rates[0][1]),
          download: Math.abs(rates[rates.length - 1][2] - rates[0][2]),
          date: new Date(rates[rates.length - 1][0]),
        });
      }
    }
    return data;
  }

  // const [selectedProcess, setSelectedProcess] = useState("");
  async function getPieCharts() {
    const connections = await invoke("gcl");
    const processes = await invoke("gpl");
    const remoteAddresses = await invoke("gral");

    // Fetch data for connections, processes, and remote addresses
    const connectionsData = await fetchData(connections, "gct", "connections");
    const processesData = await fetchData(processes, "gpt", "processes");
    const remoteAddressesData = await fetchData(
      remoteAddresses,
      "grat",
      "remoteAddresses"
    );

    console.log("connectionsData: ", connectionsData);
    console.log("processesData: ", processesData);
    console.log("remoteAddressesData: ", remoteAddressesData);

    // Set options for pie charts
    const pieDataConnections = createPieChartData(
      connectionsData,
      "Connection Download",
      "download"
    );
    const pieDataConnectionsUpload = createPieChartData(
      connectionsData,
      "Connection Upload",
      "upload"
    );
    const pieDataProcesses = createPieChartData(
      processesData,
      "Process Download",
      "download"
    );
    const pieDataProcessesUpload = createPieChartData(
      processesData,
      "Process Upload",
      "upload"
    );

    const pieDataRemoteAddresses = createPieChartData(
      remoteAddressesData,
      "Remote Address Download",
      "download"
    );
    const pieDataRAUpload = createPieChartData(
      remoteAddressesData,
      "Remote Address Upload",
      "upload"
    );

    console.log("pieDataConnections: ", pieDataConnections);
    console.log("pieDataProcesses: ", pieDataProcesses);
    console.log("pieDataRemoteAddresses: ", pieDataRemoteAddresses);
    console.log("pieDataConnectionsUpload: ", pieDataConnectionsUpload);
    console.log("pieDataProcessesUpload: ", pieDataProcessesUpload);
    console.log("pieDataRAUpload: ", pieDataRAUpload);

    // Create pie chart for connections
    const options = {
      titleTextStyle: { color: "white" },
      backgroundColor: { fill: "transparent" },
      legend: {
        textStyle: { color: "white" },
        alignment: "center",
        position: "right",
      },
      colors: [
        "#A0DDFF",
        "#758ECD",
        "#C1CEFE",
        "#7189FF",
        "#624CAB",
        "#0E217F",
      ],
    };

    // Create pie charts
    const pieChartConnections = createPieChart(
      pieDataConnections,
      "Connections",
      options
    );
    const pieChartProcesses = createPieChart(
      pieDataProcesses,
      "Processes",
      options
    );
    const pieChartRemoteAddresses = createPieChart(
      pieDataRemoteAddresses,
      "Remote Addresses",
      options
    );
    const pieChartConnectionsUpload = createPieChart(
      pieDataConnectionsUpload,
      "Connections",
      options
    );
    const pieChartProcessesUpload = createPieChart(
      pieDataProcessesUpload,
      "Processes",
      options
    );
    const pieChartRAUpload = createPieChart(
      pieDataRAUpload,
      "Remote Addresses",
      options
    );

    setPieChartConnections(pieChartConnections);
    setPieChartProcesses(pieChartProcesses);
    setPieChartRemoteAddresses(pieChartRemoteAddresses);
    setPieChartConnectionsUpload(pieChartConnectionsUpload);
    setPieChartProcessesUpload(pieChartProcessesUpload);
    setPieChartRAUpload(pieChartRAUpload);
  }

  function handleRefreshRateChange(event) {
    const { value } = event.target;
    setRefreshRate(parseInt(value));
  }

  function createPieChartData(dataList, label, dOrU) {
    let sortedData;
    if (dOrU === "download") {
      sortedData = dataList.sort((a, b) => b.download - a.download);
    } else {
      sortedData = dataList.sort((a, b) => b.upload - a.upload);
    }

    const slicedData = sortedData.slice(0, Math.min(5, sortedData.length));

    // Calculate the sum of all values beyond the top 5 elements
    const restSum = sortedData
      .slice(5)
      .reduce((acc, curr) => acc + curr[dOrU], 0);

    const pieData = [[label, dOrU === "download" ? "Download" : "Upload"]];

    // Add the top 5 elements
    for (const item of slicedData) {
      pieData.push([item.name, parseInt(item[dOrU])]);
    }

    // Add the "Others" entry with the sum of all other values
    if (sortedData.length > 5) {
      pieData.push(["Others", restSum]);
    }
    let isAllZeros = checkIfAllValuesAreZero(pieData);
    if (isAllZeros) {
      pieData.push(["All consumption is 0", 1]);
    }

    return pieData;
  }

  function createPieChart(data, title, options) {
    return (
      <Chart
        chartType="PieChart"
        data={data}
        options={{ ...options, title }}
        width="800px"
        height="320px"
      />
    );
  }

  function checkIfAllValuesAreZero(data) {
    for (let i = 1; i < data.length; i++) {
      if (data[i][1] !== 0) {
        return false;
      }
    }
    return true;
  }

  useEffect(() => {
    getPieCharts();
    // Set interval to refresh data and charts
    const interval = setInterval(getPieCharts, refreshRate);
    return () => clearInterval(interval);
  }, [refreshRate]);

  return (
    <div className="darkBackground">
      <CustomNav />
      <div>
        <Typography
          variant="body1"
          color="white"
          fontStyle="italic"
          marginLeft="20px"
          marginBottom="20px"
        >
          View processes, connections, and remote addresses by percentages of
          their total consumption in the past hour
        </Typography>
      </div>
      <div
        style={{
          marginTop: "20px",
          marginBottom: "40px",
          marginLeft: "20px",
          display: "flex",
          justifyContent: "center",
          alignItems: "center",
        }}
      >
        <Typography variant="h6" color="white">
          Refresh Rate (milliseconds):
        </Typography>
        <input
          type="number"
          value={refreshRate}
          onChange={handleRefreshRateChange}
          style={{ marginLeft: "10px" }}
        />
      </div>

      <div style={{ display: "flex", flexDirection: "column" }}>
        <div style={{ display: "flex", flexDirection: "row" }}>
          <div
            style={{
              display: "flex",
              flexDirection: "column",
              borderRight: "1px solid white",
            }}
          >
            <Typography
              sx={{ color: "white", marginLeft: "20px" }}
              variant="h6"
            >
              Download Consumption
            </Typography>
            <div>{pieChartConnections}</div>
            <div>{pieChartProcesses}</div>
            <div>{pieChartRemoteAddresses}</div>
          </div>

          {/* Second column */}
          <div style={{ display: "flex", flexDirection: "column" }}>
            <Typography
              sx={{ color: "white", marginLeft: "20px" }}
              variant="h6"
            >
              Upload Consumption
            </Typography>
            <div>{pieChartConnectionsUpload}</div>
            <div>{pieChartProcessesUpload}</div>
            <div>{pieChartRAUpload}</div>
          </div>
        </div>
      </div>
    </div>
  );
}
