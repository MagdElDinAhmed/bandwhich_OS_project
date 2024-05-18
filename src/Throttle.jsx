import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api";
import "./Tauri.css";
import CustomNav from "./Nav";
import { FormControl, Select, MenuItem, InputLabel } from "@mui/material";

export default function Throttle() {
  const [bandwidthLimit, setBandwidthLimit] = useState("");
  const [selectedOption, setSelectedOption] = useState("");
  const [interfaceList, setInterfaceList] = useState([]);

  async function gcl() {
    try {
      const connList = await invoke("gcl");
      const interfaceList = connList.map((interfaceOption) => {
        return <MenuItem value={interfaceOption}>{interfaceOption}</MenuItem>;
      });
      setInterfaceList(interfaceList);
    } catch (error) {
      console.error("Error fetching data:", error);
    }
  }

  useEffect(() => {
    gcl();
  }, []);

  const handleThrottlingThreshold = async (event) => {
    event.preventDefault();
    try {
      await invoke("throttle_bandwidth", {
        thresholdValue: parseInt(bandwidthLimit),
        interfaceName: selectedOption,
      });
      console.log(`Throttling set to ${bandwidthLimit} Mbps for ${selectedOption}`);
    } catch (error) {
      console.error("Failed to set throttling threshold:", error);
    }
  };
  const handleChange = (event) => {
    setBandwidthLimit(event.target.value);
  };

  const handleSelectInterface = (event) => {
    setSelectedOption(event.target.value);
    console.log(event.target.value);
  };
  return (
    <div className="darkBackground">
      <CustomNav />
      <div
        style={{
          display: "flex",
          justifyContent: "center",
          alignItems: "center",
          flexDirection: "column",
        }}
      >
        <div
          style={{
            display: "flex",
            flexDirection: "column",
            marginTop: "30px",
          }}
        >
          <label
            htmlFor="selectInterface"
            style={{ color: "white", marginBottom: "10px" }}
          >
            {" "}
            Set a limit in Mbps to throttle an interface:
          </label>

          <form onSubmit={handleThrottlingThreshold} style={{ width: "100%" }}>
            <div
              style={{
                display: "flex",
                flexDirection: "column",
                alignItems: "center",
                marginBottom: "20px",
              }}
            >
              <FormControl
                id="selectInterface"
                variant="filled"
                className="basicDropDown"
                style={{ width: "100%", marginRight: "10px" }}
              >
                <InputLabel style={{ color: "#1a1a1a" }}>
                  Select interface
                </InputLabel>
                <Select
                  value={selectedOption}
                  onChange={handleSelectInterface}
                  style={{ color: "#1a1a1a" }}
                >
                  <MenuItem value="" disabled>
                    Select interface
                  </MenuItem>
                  {interfaceList}
                </Select>
              </FormControl>
              <input
                type="text"
                id="bandwidthLimit"
                placeholder="Bandwidth Limit in Mbps"
                value={bandwidthLimit}
                onChange={handleChange}
                className="basicInput"
                style={{ flex: 1, marginTop: "10px" }}
              />
              <button
                type="submit"
                className="basicButton"
                style={{ marginLeft: "10px" }}
              >
                Submit
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  );
}
