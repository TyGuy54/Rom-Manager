import { GBA } from "./components/gameRoms/gba"
import { NES } from "./components/gameRoms/nes"
import { Sidenav } from "./components/sideNav";
import {
  BrowserRouter as Router,
  Routes,
  Route,
  Link
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
          {/* <Route path="/snes" element={<SNES />}/>
          <Route path="/gb" element={<GB />}/>
          <Route path="/gbc" element={<GBC />}/> */}
          <Route path="/gba" element={<GBA />}/>
        </Routes>
      </Router>
    </>
  );
}

export default App;
