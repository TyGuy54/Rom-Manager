import { useEffect, useState } from "react";
import { copyFile, BaseDirectory } from '@tauri-apps/api/fs';
import { invoke } from "@tauri-apps/api/tauri";
import { Cards } from "./components/makeCards";
import { Sidenav } from "./components/sideNav";
import "./App.css";

function App() {
  const [romData, setRomData] = useState([]);
  const [romImgs, setRomImgs] = useState([]);
  const [file, setFile] = useState();

  useEffect(() => {
    // calls a function from the back end to get data to make cards
    async function romData() {
      // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
      setRomData(await invoke("get_rom_data", {}));
    }

    // calls a function from the back end to get imgs from the backend
    async function getImgs() {
      setRomImgs(await invoke("get_img_data", {}));
    }

    romData().catch(console.error);
    getImgs().catch(console.error)
  }, []);

  const getFileExtn = () => {
    return romData.map((data) => {
      let fileExtn = data.rom_extn
      return Object.keys(fileExtn).map((extn) => {
        return fileExtn[extn]
      })
    })
  }

  
  // a funtion that copies a file from the desktop to the ROM/GBA folder
  const moveFile = async (file) => {
    // let file_name = romData[0].rom_name.NameExtn
    await copyFile(file, `Rom-Manager/ROMS/${getFileExtn()[0][0]}/${file}`, { dir: BaseDirectory.Desktop });
  }

  return (
    <>
      <div className="info-grid-container">
        {/* <h2 className="header-pos">Rom Manager</h2> */}
        <div>
          <input type="file" onChange={(e) => setFile(e.target.files[0].name)}/>
          <button className ="btn" type="submit" onClick={() => moveFile(file)}>Upload</button>
        </div>
        <Sidenav/>


        {/* <div className="wrap">
          <div className="search">
            <input type="text" className="searchTerm" placeholder="What are you looking for?"/>
            <button type="submit" className="searchButton">search</button>
          </div>
        </div> */}

        <div className="content-pos">
          <Cards 
            romImgs={romImgs} 
            cardData={romData}
            romExtn={getFileExtn()}
          />
        </div>
      </div>
    </>
  );
}

export default App;
