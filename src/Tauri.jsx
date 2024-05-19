import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api";
import "./Tauri.css";
import CustomNav from "./Nav";

function Tauri() {
  // STATES

  const [refreshRate, setRefreshRate] = useState(500); // Default refresh rate is 500ms
  const [viewProcessTotal, setViewProcessTotal] = useState(false);
  const [viewInterfaceTotal, setViewInterfaceTotal] = useState(false);
  const [viewRemoteAddressTotal, setViewRemoteAddressTotal] = useState(false);
  const [interfaceTotal, setInterfaceTotal] = useState([]);
  const [processTotal, setProcessTotal] = useState([]);
  const [remoteAddressTotal, setRemoteAddressTotal] = useState([]);
  const [tableData, setTableData] = useState(null);
  const [tableDataProcesses, setTableDataProcesses] = useState(null);
  const [tableDataRemoteAddresses, setTableDataRemoteAddresses] =
    useState(null);
  // Fetch live data
  const getLiveData = async () => {
    try {
      const liveData = await invoke("get_draw_data");
      if (liveData.length > 0) {
        setTableData(liveData[2]); // Set the interface table data
        setTableDataProcesses(liveData[1]); // Set the processes table data
        setTableDataRemoteAddresses(liveData[0]); // Set the remote addresses table data
      }
    } catch (error) {
      console.error("Error fetching live data:", error);
    }
  };

  const handleRefreshRateChange = (event) => {
    const { value } = event.target;
    setRefreshRate(parseInt(value));
  };

  useEffect(() => {
    // Fetch data initially
    getLiveData();

    // Set up a timer to fetch data periodically
    const intervalId = setInterval(() => {
      getLiveData();
    }, refreshRate); // Use refreshRate as interval duration

    // Clear the timer when the component unmounts
    return () => clearInterval(intervalId);
  }, [refreshRate]); // Include refreshRate in the dependency array

  async function gcl() {
    try {
      const connList = await invoke("gcl");
      const interfaceList = connList.map((interfaceOption) => {
        return (
          <option key={interfaceOption} value={interfaceOption}>
            {interfaceOption}
          </option>
        );
      });
      setInterfaceList(interfaceList);
    } catch (error) {
      console.error("Error fetching data:", error);
    }
  }

  async function gpl() {
    try {
      const procList = await invoke("gpl");
      setProcessList(procList);
      document.getElementById("myDropdown").classList.toggle("show");
    } catch (error) {
      console.error("Error fetching data:", error);
    }
  }

  async function displayInterfacesTotal() {
    setViewInterfaceTotal(!viewInterfaceTotal);
    setViewProcessTotal(false);
    setViewRemoteAddressTotal(false);

    if (!viewInterfaceTotal) {
      const connections = await invoke("gpl");
      const interfaceRows = [];
      for (const connection of connections) {
        const rates = await invoke("gpt", {
          process: connection,
          time: "Last Hour",
        });
        for (let j = rates.length - 1; j >= 0; j--) {
          const timestamp = new Date(rates[j][0]);
          const formattedDateTime = timestamp.toLocaleString();
          interfaceRows.push(
            <tr key={`${connection}-${j}`}>
              <td>{connection}</td>
              <td>{rates[j] ? formattedDateTime : "Loading..."}</td>
              <td>{rates[j] ? rates[j][1] : "Loading..."}</td>
              <td>{rates[j] ? rates[j][2] : "Loading..."}</td>
            </tr>
          );
        }
      }
      setInterfaceTotal(interfaceRows);
    }
  }

  async function displayProcessesTotal() {
    setViewProcessTotal(!viewProcessTotal);
    setViewInterfaceTotal(false);
    setViewRemoteAddressTotal(false);

    if (!viewProcessTotal) {
      const processes = await invoke("gpl");
      const processRows = [];
      for (const process of processes) {
        const rates = await invoke("gpr", {
          process: process,
          time: "Last Hour",
        });
        for (let j = rates.length - 1; j >= 0; j--) {
          const timestamp = new Date(rates[j][0]);
          const formattedDateTime = timestamp.toLocaleString();
          processRows.push(
            <tr key={`${process}-${j}`}>
              <td>{process}</td>
              <td>{rates[j] ? formattedDateTime : "Loading..."}</td>
              <td>{rates[j] ? rates[j][1] : "Loading..."}</td>
              <td>{rates[j] ? rates[j][2] : "Loading..."}</td>
            </tr>
          );
        }
      }
      setProcessTotal(processRows);
    }
  }

  async function displayRemoteAddressesTotal() {
    setViewRemoteAddressTotal(!viewRemoteAddressTotal);
    setViewInterfaceTotal(false);
    setViewProcessTotal(false);

    if (!viewRemoteAddressTotal) {
      const remoteAddresses = await invoke("gra");
      const remoteAddressRows = [];
      for (const remoteAddress of remoteAddresses) {
        const rates = await invoke("grr", {
          remoteAddress: remoteAddress,
          time: "Last Hour",
        });
        for (let j = rates.length - 1; j >= 0; j--) {
          const timestamp = new Date(rates[j][0]);
          const formattedDateTime = timestamp.toLocaleString();
          remoteAddressRows.push(
            <tr key={`${remoteAddress}-${j}`}>
              <td>{remoteAddress}</td>
              <td>{rates[j] ? formattedDateTime : "Loading..."}</td>
              <td>{rates[j] ? rates[j][1] : "Loading..."}</td>
              <td>{rates[j] ? rates[j][2] : "Loading..."}</td>
            </tr>
          );
        }
      }
      setRemoteAddressTotal(remoteAddressRows);
    }
  }

  useEffect(() => {
    gcl();
  }, []);

  return (
    <div className="darkBackground">
      <CustomNav />
      <div
        style={{
          display: "flex",
          justifyContent: "center",
          alignItems: "center",
          flexDirection: "column",
          color: "white",
          fontStyle: "italic",
        }}
      >
        View Live Consumption Data
      </div>
      <div
        style={{
          marginTop: "10px",
          marginBottom: "10px",
          display: "flex",
          justifyContent: "center",
          alignItems: "center",
        }}
      >
        <label htmlFor="refreshRateInput" style={{ color: "white" }}>
          Refresh Rate (milliseconds):
        </label>
        <input
          type="number"
          id="refreshRateInput"
          value={refreshRate}
          onChange={handleRefreshRateChange}
          style={{ marginLeft: "10px" }}
        />
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
      {viewProcessTotal && (
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
                <th>Process</th>
                <th>Time Stamp</th>
                <th>Upload Rate</th>
                <th>Download Rate</th>
              </tr>
            </thead>
            <tbody>{processTotal}</tbody>
          </table>
        </div>
      )}
      {viewRemoteAddressTotal && (
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
                <th>Remote Address</th>
                <th>Time Stamp</th>
                <th>Upload Rate</th>
                <th>Download Rate</th>
              </tr>
            </thead>
            <tbody>{remoteAddressTotal}</tbody>
          </table>
        </div>
      )}
      <div
        style={{
          minWidth: "100vw",
          display: "flex",
          justifyContent: "center",
          alignItems: "center",
          flexDirection: "column",
        }}
      >
        <div
          style={{
            width: "1000px",
            marginBottom: "120px",
          }}
        >
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
                display: "flex",
                justifyContent: "center",
              }}
            >
              <div>
                <h2 style={{ color: "white", marginLeft: "30px" }}>
                  {tableData.title}
                </h2>
                <table
                  style={{
                    color: "white",
                    textAlign: "center",
                    tableLayout: "fixed",
                    width: "100%",
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
            </div>
          )}
          {tableDataProcesses && (
            <div
              style={{
                color: "white",
                marginLeft: "30px",
                marginRight: "30px",
                marginTop: "20px",
                maxHeight: "300px",
                overflowY: "auto",
                border: "1px solid white",
                display: "flex",
                justifyContent: "center",
              }}
            >
              <div>
                <h2 style={{ color: "white", marginLeft: "30px" }}>
                  {tableDataProcesses.title}
                </h2>
                <table
                  style={{
                    color: "white",
                    textAlign: "center",
                    tableLayout: "fixed",
                    width: "100%",
                  }}
                >
                  <thead>
                    <tr>
                      {tableDataProcesses.column_names.map(
                        (columnName, index) => (
                          <th key={index} style={{ color: "white" }}>
                            {columnName}
                          </th>
                        )
                      )}
                    </tr>
                  </thead>
                  <tbody>
                    {tableDataProcesses.rows.map((row, rowIndex) => (
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
            </div>
          )}
          {tableDataRemoteAddresses && (
            <div
              style={{
                color: "white",
                marginLeft: "30px",
                marginRight: "30px",
                marginTop: "20px",
                maxHeight: "300px",
                overflowY: "auto",
                border: "1px solid white",
                display: "flex",
                justifyContent: "center",
              }}
            >
              <div>
                <h2 style={{ color: "white", marginLeft: "30px" }}>
                  {tableDataRemoteAddresses.title}
                </h2>
                <table
                  style={{
                    color: "white",
                    textAlign: "center",
                    tableLayout: "fixed",
                    width: "100%",
                  }}
                >
                  <thead>
                    <tr>
                      {tableDataRemoteAddresses.column_names.map(
                        (columnName, index) => (
                          <th key={index} style={{ color: "white" }}>
                            {columnName}
                          </th>
                        )
                      )}
                    </tr>
                  </thead>
                  <tbody>
                    {tableDataRemoteAddresses.rows.map((row, rowIndex) => (
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
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

export default Tauri;
