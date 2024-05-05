import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [process_list, setProcessList] = useState(["Item1"]);
  const [process_rates, setProcessRates] = useState([["Hello", "World", "How"]]);
  const [selectedProcess, setSelectedProcess] = useState("");
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

  async function gpr() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    
    
    const rates = await invoke("gpr", {process: selectedProcess, time: selectedTime});
    setGreetMsg("Process Rates");
    const processRates = rates.map(rate => rate.map(item => item));
    
    setProcessRates(processRates);

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
      
      <p>Selected Process: {selectedProcess}</p>
      <p>Selected Time: {selectedTime}</p>
      
      <div className="button-container">
        <button onClick={() => setSelectedTime("All time")}>All time</button>
        <button onClick={() => setSelectedTime("Last Year")}>Last Year</button>
        <button onClick={() => setSelectedTime("Last Month")}>Last Month</button>
        <button onClick={() => setSelectedTime("Last Week")}>Last Week</button>
        <button onClick={() => setSelectedTime("Last Day")}>Last Day</button>
      </div>

      <div class="dropdown">
        <button onClick={gpl} class="dropbtn">Dropdown</button>
        <div id="myDropdown" class="dropdown-content">
          
          {process_list.map((item, index) => (
            <a key={index} onClick={() => setSelectedProcess(item)}>{item}</a>
          ))}
           
        </div>
      </div>

      <button onClick={gpr}>Display Process Rates</button>
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
            {process_rates.map((rate, index) => (
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
