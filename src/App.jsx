import { useEffect, useState } from "react";
import { copyFile, BaseDirectory } from '@tauri-apps/api/fs';
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import "./card.css";

function App() {
  const [cardDatas, setCardData] = useState([]);
  const [romImgs, setRomImgs] = useState([]);
  const [file, setFile] = useState();

  useEffect(() => {
    async function cardData() {
      // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
      setCardData(await invoke("card_data", {}));
    }

    async function getImgs() {
      setRomImgs(await invoke("get_imgs", {}));
    }

    cardData().catch(console.error);
    getImgs().catch(console.error)
  }, []);

  async function loadRom(romName) {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    await invoke("load_rom", { romName })
      .then((data) => {
        console.log(`This is from loadRom ${data}`);
      })
      .catch((error) => {
        console.log(error);
      });
  }

  const setImg = (romName) => {
    return romImgs.map((obj, i) => {
      let img_path = obj.img_path;
      let img_name = obj.img_name;
      return Object.keys(img_name).map((img, i) => {
        return Object.keys(img_path).map((pth, i) => {
          if (img_name[img].replace(".jpg", "") == romName.replace(".gba", "")) {
            return (
              <img
                src={img_path[pth]}
                className="card__image"
                alt="rom img"
              />
            );
          }
        });
      });
    });
  }

  const makeCards = () => {
    return cardDatas.map((data, i) => {
      let path = data.rom_path;
      let name = data.rom_name;
      return Object.keys(path).map((obj, i) => {
        return (
          <>
            <li>
              <div
                className="card"
                key={i}
                onClick={(e) => {
                  e.preventDefault();
                  loadRom(path[obj]);
                  console.log(path[obj])
                }}
              >
                {Object.keys(name).map((obj, i) => {
                  return (
                    setImg(name[obj])
                  );
                })}
                <div className="card__overlay">
                  <div className="card__header">
                    <div className="card__header-text">
                      <h3 className="card__title">Game Boy Advanced</h3>
                    </div>
                  </div>
                  <div className="container">
                    {Object.keys(name).map((obj, i) => {
                      return <h4>{name[obj]}</h4>;
                    })}
                  </div>
                </div>
              </div>
            </li>
          </>
        );
      });
    });
  };

const move_file = async (file) => {
  await copyFile(file, `Rom-Manager/ROMS/GBA/${file}`, { dir: BaseDirectory.Desktop });
 }

  return (
    <>
      <div className="info-grid-container">
        <h2 className="header-pos">Rom Manager</h2>
        <nav>
          <input type="file" onChange={(e) => setFile(e.target.files[0].name)}/>
          <button type="submit" onClick={() => move_file(file)}>Upload</button>
          <a href="#">about</a>
        </nav>

        {/* <div className="wrap">
          <div className="search">
            <input type="text" className="searchTerm" placeholder="What are you looking for?"/>
            <button type="submit" className="searchButton">search</button>
          </div>
        </div> */}

        <div className="content-pos">
          <ul className="cards">{makeCards()}</ul>
          {/* <button onClick={(e) => move_file()}>click me</button> */}
        </div>
      </div>
    </>
  );
}

export default App;
