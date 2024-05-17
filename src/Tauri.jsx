import { useState, useEffect } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api";
import { Button, Typography, TextField } from "@mui/material";
// import textfield from mui
// import TextField from '@mui/material/TextField';
import "./Tauri.css";
// import { PieChart } from "@mui/x-charts/PieChart";
import { PieChart, Pie, Legend, Tooltip, ResponsiveContainer } from "recharts";
import { Chart } from "react-google-charts";

function Tauri() {
  // STATES
  const [bandwidthLimit, setBandwidthLimit] = useState("");
  const [selectedOption, setSelectedOption] = useState("");
  const [interfaceList, setInterfaceList] = useState([]);
  const [process_list, setProcessList] = useState([]);
  const [viewProcessRates, setViewProcessRates] = useState(false);
  const [viewInterfaceRates, setViewInterfaceRates] = useState(false);
  const [viewRemoteAddressRates, setViewRemoteAddressRates] = useState(false);
  const [viewProcessTotal, setViewProcessTotal] = useState(false);
  const [viewInterfaceTotal, setViewInterfaceTotal] = useState(false);
  const [viewRemoteAddressTotal, setViewRemoteAddressTotal] = useState(false);
  const [interfaceTotal, setInterfaceTotal] = useState([]);
  const [isTotal, setIsTotal] = useState(false);
  const [pieChart, setPieChart] = useState();
  const [pieChartConnections, setPieChartConnections] = useState();
  const [pieChartProcesses, setPieChartProcesses] = useState();
  const [pieChartRemoteAddresses, setPieChartRemoteAddresses] = useState();

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
  async function getPieCharts2() {
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
    const pieDataProcesses = createPieChartData(
      processesData,
      "Process Download",
      "download"
    );
    const pieDataRemoteAddresses = createPieChartData(
      remoteAddressesData,
      "Remote Address Download",
      "download"
    );

    console.log("pieDataConnections: ", pieDataConnections);
    console.log("pieDataProcesses: ", pieDataProcesses);
    console.log("pieDataRemoteAddresses: ", pieDataRemoteAddresses);

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
      "Connections Download",
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

    setPieChartConnections(pieChartConnections);
    setPieChartProcesses(pieChartProcesses);
    setPieChartRemoteAddresses(pieChartRemoteAddresses);
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

  async function getPieCharts() {
    const connections = await invoke("gpl");
    // console.log(connections);
    let connectionsData = [];
    for (let i = 0; i < 2; i++) {
      const connection = connections[i];
      const rates = await invoke("gpt", {
        process: connection,
        time: "Last Hour",
      });
      console.log("Curr connection: ", connection);
      console.log("Curr totals: ", rates);
      let latest = rates[rates.length - 1][0];
      if (latest == "No data available") {
        continue;
      }
      console.log("Latest: ", rates[rates.length - 1]);
      const temp = {
        name: connection,
        date: new Date(latest),
        upload: rates[rates.length - 1][1] - rates[0][1],
        download: rates[rates.length - 1][2] - rates[0][2],
      };
      connectionsData.push(temp);
      // console.log("connectionsData:", connectionsData);
    }
    console.log("connectionsData", connectionsData);
    // take 5 connections with the highest download rates
    connectionsData.sort((a, b) => b.download - a.download);
    connectionsData = connectionsData.slice(
      0,
      Math.max(5, connectionsData.length)
    );
    let pieData1 = [["Connection", "Download"]];
    for (let i = 0; i < connectionsData.length; i++) {
      const connectionName = connectionsData[i].name;
      const download = parseInt(connectionsData[i].download);
      const arrayElement = [connectionName, download];
      pieData1.push(arrayElement);
    }
    // console.log("pieData1: ", pieData1);

    const options = {
      title: "Connections Download",
      titleTextStyle: {
        color: "white", // Color of the title text
      },
      backgroundColor: { fill: "transparent" },
      legend: {
        textStyle: {
          color: "white", // Change label color here
        },
        alignment: "center", // Alignment of the legend: "start", "center", or "end"
        position: "right", // Position of the legend: "top", "bottom", "left", "right", or "none"
      },
      colors: ["#87CEEB", "#FFED65", "#629460", "#386641", "#FF0000"],
    };
    const pieChart = (
      <Chart
        chartType="PieChart"
        data={pieData1}
        options={options}
        width={"900px"}
        height={"400px"}
      />
    );
    const pieData = connectionsData.map((connection) => {
      return {
        name: connection.name,
        value: parseInt(connection.download),
      };
    });
    console.log(pieData);

    console.log(pieChart);
    setPieChart(pieChart);
  }

  // FUNCTIONS
  async function gcl() {
    try {
      const connList = await invoke("gcl");
      const interfaceList = connList.map((interfaceOption) => {
        return <option value={interfaceOption}>{interfaceOption}</option>;
      });
      setInterfaceList(interfaceList);
    } catch (error) {
      console.error("Error fetching data:", error);
    }
  }
  async function gpl() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    const proc_list = await invoke("gpl");
    setProcessList(proc_list);
    document.getElementById("myDropdown").classList.toggle("show");
  }
  async function gpr() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    const rates = await invoke("gpr", {
      process: selectedProcess,
      time: selectedTime,
    });
    setGreetMsg("Process Rates");
    const processRates = rates.map((rate) => rate.map((item) => item));
    setProcessRates(processRates);
  }
  async function handleThrottlingThreshold(event) {
    event.preventDefault();
    await invoke("get_throttling_threshold", {
      thresholdValue: parseInt(bandwidthLimit),
    });
  }
  async function displayInterfacesTotal() {
    if (viewInterfaceTotal) {
      setViewInterfaceTotal(false);
    } else {
      setViewInterfaceTotal(true);
      setViewInterfaceRates(false);
      setViewRemoteAddressRates(false);
      setViewProcessRates(false);
      setViewInterfaceTotal(true);
      setViewProcessTotal(false);
      setViewRemoteAddressTotal(false);
      console.log("HEREEEE");

      const connections = await invoke("gpl");
      // console.log(connections);
      const interfaceRows = [];
      for (let i = 0; i < connections.length; i++) {
        const connection = connections[i];
        const rates = await invoke("gpt", {
          process: connection,
          time: "Last Hour",
        });
        // for (let j = 0; j < rates.length; j++) {
        for (let j = rates.length - 1; j >= 0; j--) {
          const timestamp = new Date(rates[j][0]);
          // Format the date and time
          const formattedDateTime = timestamp.toLocaleString();
          interfaceRows.push(
            <tr>
              <td>{connection}</td>
              <td>{rates[j] ? formattedDateTime : "Loading..."}</td>
              <td>{rates[j] ? rates[j][1] : "Loading..."}</td>
              <td>{rates[j] ? rates[j][2] : "Loading..."}</td>
            </tr>
          );
        }
        setInterfaceTotal(interfaceRows);
        // console.log("interfaceTotal: ", interfaceTotal);
        console.log("Connection: ", connection);
        console.log("Rates: ", rates);
      }
    }
  }
  async function displaySomething() {
    const processUtilization = await invoke("get_process_utilization");
    console.log(processUtilization);
  }

  const getProcesses = async () => {
    setViewInterfaceRates(false);
    setViewRemoteAddressRates(false);
    setViewProcessRates(true);

    const processes = await invoke("gpl");
    console.log(processes);
    for (let i = 0; i < processes.length; i++) {
      // call gpr for each process
      const process = processes[i];
      const rates = await invoke("gpr", {
        process: process,
        time: "Last Second",
      });
      console.log("Process: ", process);
      console.log("Rates: ", rates);
    }
  };

  const handleChange = (event) => {
    setBandwidthLimit(event.target.value);
  };
  const handleSelectInterface = (event) => {
    setSelectedOption(event.target.value);
    console.log(event.target.value);
  };

  // EFFECTS
  useEffect(() => {
    gcl();
  }, []);

  return (
    <div className="darkBackground">
      <div
        style={{
          display: "flex",
          flexDirection: "column",
        }}
      >
        <button onClick={getPieCharts}>Get Pie Charts</button>
        <button onClick={getPieCharts2}>Get Pie Charts 2</button>
        {/* {(pieChartConnections, pieChartProcesses, pieChartRemoteAddresses)} */}
        <div
          style={{
            maxWidth: "100vw",
            marginLeft: "0",
            paddingLeft: "0",
            display: "grid",
            gridTemplateColumns: "repeat(auto-fit, minmax(300px, 1fr))", // Adjust column width as needed
            // gap: "20px", // Adjust gap between charts
            gap: "0",
            justifyContent: "center", // Center the charts horizontally
          }}
        >
          {/* First Pie Chart */}
          <div>{pieChartConnections}</div>

          {/* Second Pie Chart */}
          <div>{pieChartProcesses}</div>

          {/* Third Pie Chart */}
          <div>{pieChartRemoteAddresses}</div>
        </div>
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
        <form
          onSubmit={handleThrottlingThreshold}
          style={{ marginLeft: "30px", marginTop: "5px" }}
        >
          <div
            style={{
              display: "flex",
              flexDirection: "column",
              marginTop: "20px",
            }}
          >
            <label htmlFor="bandwidthLimit" style={{ color: "white" }}>
              {" "}
              Set a limit in Mbps to throttle an interface:
            </label>
            <div style={{ display: "flex", flexDirection: "row" }}>
              <input
                type="text"
                id="bandwidthLimit"
                placeholder="Bandwidth Limit in Mbps"
                value={bandwidthLimit}
                onChange={handleChange}
                className="basicInput"
              />
              <button type="submit" className="basicButton">
                Submit
              </button>
            </div>
          </div>
        </form>
        <form style={{ marginLeft: "30px", marginTop: "5px" }}>
          <select
            value={selectedOption}
            onChange={handleSelectInterface}
            style={{ padding: "0px 7px 0 7px" }}
          >
            <option value="" disabled selected>
              Select interface
            </option>
            {interfaceList}
          </select>
        </form>
        <div style={{ color: "white", marginLeft: "30px", marginTop: "5px" }}>
          View past data for:
        </div>
        {/* choose between interfaces, processes, remote addresses*/}
        <div
          style={{
            display: "flex",
            flexDirection: "row",
            marginLeft: "30px",
            marginTop: "5px",
          }}
        >
          <button onClick={displayInterfacesTotal} className="basicButton">
            Interfaces
          </button>
          <button className="basicButton">Processes</button>
          <button className="basicButton">Remote Addresses</button>
        </div>
        {viewInterfaceTotal && (
          <div
            style={{
              color: "white",
              marginLeft: "30px",
              marginRight: "30px",
              marginTop: "5px",
              maxHeight: "300px",
              overflowY: "auto",
              border: "1px solid white",
            }}
          >
            <table
              style={{
                color: "white",
                marginLeft: "100px",
                marginRight: "100px",
                maxHeight: "300px",
                textAlign: "center",
              }}
            >
              <thead>
                <tr>
                  <th sx={{ color: "white" }}>Connection</th>
                  <th>Time Stamp</th>
                  <th>Upload Rate</th>
                  <th>Download Rate</th>
                </tr>
              </thead>
              <tbody>{interfaceTotal}</tbody>
            </table>
          </div>
        )}
      </div>
    </div>
  );
}
export default Tauri;
