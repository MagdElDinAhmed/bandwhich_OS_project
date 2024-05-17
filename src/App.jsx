import "./App.css";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import Tauri from "./Tauri";
import PieCharts from "./PieCharts";
import LineGraphs from "./LineGraphs";

function App() {
  return (
    <div>
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<Tauri />} />
          <Route path="/PieCharts" element={<PieCharts />} />
          <Route path="/LineGraphs" element={<LineGraphs />} />
        </Routes>
      </BrowserRouter>
    </div>
  );
}
export default App;
