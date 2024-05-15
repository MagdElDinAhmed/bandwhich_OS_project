import { useState, useEffect } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api";
import { Button, Typography, TextField } from "@mui/material";
// import textfield from mui
// import TextField from '@mui/material/TextField';
import "./Tauri.css";

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

  // const [selectedProcess, setSelectedProcess] = useState("");

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

      const connections = await invoke("gcl");
      // console.log(connections);
      const interfaceRows = [];
      for (let i = 0; i < connections.length; i++) {
        const connection = connections[i];
        const rates = await invoke("gcr", {
          connection: connection,
          time: "Last Hour",
        });
        // for (let j = 0; j < rates.length; j++) {
        for (let j = rates.length - 1; j >= 0; j--) {
          const timestamp = new Date(rates[j][0]);
          // Format the date and time
          const formattedDateTime = timestamp.toLocaleString();
          interfaceRows.push(
            <tr key={i + j}>
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
