import { GBA } from "./components/gameRoms/gba"
import { GB } from "./components/gameRoms/gb"
import { GBC } from "./components/gameRoms/gbc"
import { NES } from "./components/gameRoms/nes"
import { SNES } from "./components/gameRoms/snes"
import { Sidenav } from "./components/sideNav";
import {
  BrowserRouter as Router,
  Routes,
  Route
} from "react-router-dom";
import "./App.css";
import { LandingPage } from "./components/landing";


function App() {
  return (
    <>
      <Router>
        <Sidenav />
        <Routes>
          <Route path="/" element={<LandingPage />}/>
          <Route path="/nes" element={<NES />}/>
          <Route path="/snes" element={<SNES />}/>
          <Route path="/gb" element={<GB />}/>
          <Route path="/gbc" element={<GBC />}/>
          <Route path="/gba" element={<GBA />}/>
        </Routes>
      </Router>
    </>
  );
}

export default App;
