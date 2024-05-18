import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api";
import "./Tauri.css";
import CustomNav from "./Nav";

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
  const [interfaceTotal, setInterfaceTotal] = useState([]);
  const [isTotal, setIsTotal] = useState(false);
  const [tableData, setTableData] = useState(null);
  // const [selectedProcess, setSelectedProcess] = useState("");

  //

  const getLiveData = async () => {
    try {
      const liveData = await invoke("get_draw_data");
      if (liveData.length > 0) {
        setTableData(liveData[2]); // Set the first table data
      }
    } catch (error) {
      console.error("Error fetching live data:", error);
    }
  };

  useEffect(() => {
    // Fetch data initially
    getLiveData();

    // Set up a timer to fetch data periodically
    const intervalId = setInterval(() => {
      getLiveData();
    }, 500); // Adjust the interval as needed (5000ms = 5 seconds)

    // Clear the timer when the component unmounts
    return () => clearInterval(intervalId);
  }, []);

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
      // setViewRemoteAddressTotal(false);
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
      <CustomNav />
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
                <th>Connection</th>
                <th>Time Stamp</th>
                <th>Upload Rate</th>
                <th>Download Rate</th>
              </tr>
            </thead>
            <tbody>{interfaceTotal}</tbody>
          </table>
        </div>
      )}
      {/* Render the table fetched from the backend */}
      {tableData && (
        <div
          style={{
            color: "white",
            marginLeft: "30px",
            marginRight: "30px",
            marginTop: "20px",
            maxHeight: "300px",
            overflowY: "auto",
            border: "1px solid white",
          }}
        >
          <h2 style={{ color: "white" }}>{tableData.title}</h2>
          <table
            style={{
              color: "white",
              marginLeft: "30px",
              marginRight: "30px",
              textAlign: "center",
            }}
          >
            <thead>
              <tr>
                {tableData.column_names.map((columnName, index) => (
                  <th key={index} style={{ color: "white" }}>
                    {columnName}
                  </th>
                ))}
              </tr>
            </thead>
            <tbody>
              {tableData.rows.map((row, rowIndex) => (
                <tr key={rowIndex}>
                  {row.map((cell, cellIndex) => (
                    <td key={cellIndex} style={{ color: "white" }}>
                      {cell}
                    </td>
                  ))}
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
}

export default Tauri;
