import "./App.css";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import Tauri from "./Tauri";
import PieCharts from "./PieCharts";

function App() {
  return (
    <div>
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<Tauri />} />
          <Route path="/PieCharts" element={<PieCharts />} />
        </Routes>
      </BrowserRouter>
    </div>
  );
}
export default App;
