import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [process_list, setProcessList] = useState(["Item1"]);
  const [connection_list, setConnectionList] = useState(["Item1"]);
  const [remote_address_list, setRemoteAddressList] = useState(["Item1"]);

  const [process_rates, setProcessRates] = useState([["Hello", "World", "How"]]);
  const [connection_rates, setConnectionRates] = useState([["Hello", "World", "How"]]);
  const [remote_address_rates, setRemoteAddressRates] = useState([["Hello", "World", "How"]]);

  const [process_totals, setProcessTotals] = useState([["Hello", "World", "How"]]);
  const [connection_totals, setConnectionTotals] = useState([["Hello", "World", "How"]]);
  const [remote_address_totals, setRemoteAddressTotals] = useState([["Hello", "World", "How"]]);
  
  const [selectedProcess, setSelectedProcess] = useState("");
  const [selectedConnection, setSelectedConnection] = useState("");
  const [selectedRemoteAddress, setSelectedRemoteAddress] = useState("");

  const [selectedTime, setSelectedTime] = useState("All time");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  
  async function gpl() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    const proc_list = await invoke("gpl");
    setProcessList(proc_list);
    document.getElementById("myDropdown").classList.toggle("show");
  }

  async function gcl() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    const conn_list = await invoke("gcl");
    setConnectionList(conn_list);
    document.getElementById("myDropdown").classList.toggle("show");
  }

  async function gral() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    const ra_list = await invoke("gral");
    setRemoteAddressList(ra_list);
    document.getElementById("myDropdown").classList.toggle("show");
  }

  async function gpr() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    const rates = await invoke("gpr", {process: selectedProcess, time: selectedTime});
    setGreetMsg("Process Rates");
    const processRates = rates.map(rate => rate.map(item => item));
    setProcessRates(processRates);
  }

  async function gcr() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    const rates = await invoke("gcr", {connection: selectedConnection, time: selectedTime});
    setGreetMsg("Connection Rates");
    const connectionRates = rates.map(rate => rate.map(item => item));
    setConnectionRates(connectionRates);
  }

  async function grar() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    const rates = await invoke("grar", {remote_address: selectedRemoteAddress, time: selectedTime});
    setGreetMsg("Remote Address Rates");
    const remoteAddressRates = rates.map(rate => rate.map(item => item));
    setRemoteAddressRates(remoteAddressRates);
  }

  async function gpt() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    const totals = await invoke("gpt", {process: selectedProcess, time: selectedTime});
    setGreetMsg("Process Totals");
    const processTotals = totals.map(total => total.map(item => item));
    setProcessTotals(processTotals);
  }

  async function gct() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    const totals = await invoke("gct", {connection: selectedConnection, time: selectedTime});
    setGreetMsg("Connection Totals");
    const connectionTotals = totals.map(total => total.map(item => item));
    setConnectionTotals(connectionTotals);
  }

  async function grat() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    const totals = await invoke("grat", {remote_address: selectedRemoteAddress, time: selectedTime});
    setGreetMsg("Remote Address Totals");
    const remoteAddressTotals = totals.map(total => total.map(item => item));
    setRemoteAddressTotals(remoteAddressTotals);
  }

  

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>

      <p>{greetMsg}</p>
      
      <p>Selected Process: {selectedConnection}</p>
      <p>Selected Time: {selectedTime}</p>
      
      <div className="button-container">
        <button onClick={() => setSelectedTime("All time")}>All time</button>
        <button onClick={() => setSelectedTime("Last Year")}>Last Year</button>
        <button onClick={() => setSelectedTime("Last Month")}>Last Month</button>
        <button onClick={() => setSelectedTime("Last Week")}>Last Week</button>
        <button onClick={() => setSelectedTime("Last Day")}>Last Day</button>
      </div>

      <div class="dropdown">
        <button onClick={gcl} class="dropbtn">Dropdown</button>
        <div id="myDropdown" class="dropdown-content">
          
          {connection_list.map((item, index) => (
            <a key={index} onClick={() => setSelectedConnection(item)}>{item}</a>
          ))}
           
        </div>
      </div>

      <button onClick={gct}>Display Process Rates</button>
      <div className="process-rates">
        <table>
          <thead>
            <tr>
              <th>Time</th>
              <th>Upload</th>
              <th>Download</th>
            </tr>
          </thead>
          <tbody>
            {connection_totals.map((rate, index) => (
              <tr key={index}>
                <td>{rate[0]}</td>
                <td>{rate[1]}</td>
                <td>{rate[2]}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      

    </div>
  );
}

export default App;
