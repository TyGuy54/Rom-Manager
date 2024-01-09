import { useEffect, useState } from "react";
import { IconContext } from "react-icons";
import * as FaIcons from "react-icons/fa";
import * as AiIcons from "react-icons/ai";
import { SidebarData } from "./lib/sideNavData";
import {
  BrowserRouter as Router,
  Route,
  Link
} from "react-router-dom";
import "./css/nav.css";

export const Sidenav = () => {
  const [toggle, setToggle] = useState(false);

  const handleClick = () => {
    setToggle(!toggle);
  };

  return (
    <>
      <IconContext.Provider value={{ color: "#fff" }}>
        <div className="navbar">
          <a href="#" className="menu-bars">
            <FaIcons.FaBars onClick={handleClick} />
          </a>
          <div style={{margin: "0 auto"}}>
            <p>Rom Manager</p>
          </div>
        </div>
        <nav className={toggle ? "nav-menu active" : "nav-menu"}>
          <ul className="nav-menu-items" onClick={handleClick}>
            <li className="navbar-toggle">
              <a href="#" className="menu-bars">
                <AiIcons.AiOutlineClose />
              </a>
            </li>
            {SidebarData.map((item, index) => {
              return (
                <li key={index} className={item.cName}>
                  <Link to={item.path}>
                    {/* {item.icon} */}
                    <span>{item.title}</span>
                  </Link>
                </li>
              );
            })}
          </ul>
        </nav>
      </IconContext.Provider>
    </>
  );
};
