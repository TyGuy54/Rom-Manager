import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [cardDatas, setCardData] = useState([]);

  useEffect(() => {
    async function cardData() {
      // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
      setCardData(await invoke("card_data", {}))
    }

    cardData().catch(console.error)
  }, [])

  async function loadRom(romName) {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    await invoke("load_rom", { romName }).then((data) => {
      console.log(`This is from loadRom ${data}`)
    }
    ) .catch((error) => {
        console.log(error);
      }
    );
  }

  const makeCards = () => {
    return cardDatas.map((data, i) => {
      let path = data.rom_path
      let name = data.rom_name
      return Object.keys(path).map((obj, i) => {
          return(
            <div className="card" key={i} onClick={(e) => {
              e.preventDefault();
              loadRom(path[obj]);
            }}>
              <div className="container">
                  {Object.keys(name).map((obj, i) => {
                    return <h4>{name[obj]}</h4>
                  })}
              </div>
          </div> 
          );
        })
    })
  }

  return (
    <div className="info-grid-container">
      <h2 className="header-pos">Rom Manager</h2>

      <div className="wrap">
      <div className="search">
          <input type="text" className="searchTerm" placeholder="What are you looking for?"/>
          <button type="submit" className="searchButton">s</button>
      </div>
    </div>

      <div className="content-pos">
          {makeCards()}
      </div>
    </div>
  );
}

export default App;