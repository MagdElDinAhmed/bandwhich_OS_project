import "./App.css";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import Tauri from "./Tauri";
import PieCharts from "./PieCharts";
import LineGraphs from "./LineGraphs";
import Throttle from "./Throttle";

function App() {
  return (
    <div>
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<Tauri />} />
          <Route path="/PieCharts" element={<PieCharts />} />
          <Route path="/LineGraphs" element={<LineGraphs />} />
          <Route path="/Throttle" element={<Throttle />} />
        </Routes>
      </BrowserRouter>
    </div>
  );
}
export default App;
