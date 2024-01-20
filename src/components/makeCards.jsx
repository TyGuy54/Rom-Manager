import { loadRom } from "./lib/loadRom";
import "./css/card.css";

export const Cards = (props) => {
  // a function to set the roms box art image based on the roms name
  // for the time being it has to be a .jpg
  // the .jpg has to have the same name as the rom
  const setImg = (romName) => {
    return props.romImgs.map((obj) => {
      let img_path = obj.img_path;
      let img_name = obj.img_name;
      return Object.keys(img_name).map((img) => {
        return Object.keys(img_path).map((pth, i) => {
          let extn = props.romExtn
          if (img_name[img].replace(".jpg", "") == romName.replace(`.${extn[0]["NameExtn"].toLowerCase()}`, "")) {
            return (
              <img
                src={img_path[pth]}
                className="card__image"
                alt="rom image"
                key={i}
              />
            );
          }
        });
      });
    });
  };

  // a function that takes an array[object] from the back end and uses that data to make cards for the rom
  const makeCards = () => {
    return props.cardData.map((data, i) => {
      let fileExtn = data.file_extension["NameExtn"]
      let path = data.file_location;
      let name = data.file_name;
      // I have to use [Object.keys()] becuase when useing the object in the array
      // its looks likes this -> {String: "some roms name"} and useing dot notation
      // throws an error of undifind.
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
                  props.roms;
                }}
              >
                {Object.keys(name).map((obj) => {
                  return setImg(name[obj]);
                })}
                <div className="card__overlay" key={i}>
                  <div className="card__header">
                    <div className="card__header-text">
                      <h3 className="card__title">{fileExtn}</h3>
                    </div>
                  </div>
                  <div className="container">
                    {Object.keys(name).map((obj, i) => {
                      return <h4 key={i}>{name[obj]}</h4>;
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

  return (
    <>
      <ul className="cards">{makeCards()}</ul>
    </>
  );
};
