import React from 'react';
import * as FaIcons from 'react-icons/fa';
import * as AiIcons from 'react-icons/ai';
import * as IoIcons from 'react-icons/io';

export const SidebarData = [
  {
    title: 'Home',
    path: '/',
    // icon: <AiIcons.AiFillHome />,
    cName: 'nav-text'
  },
  {
    title: 'NES',
    path: '/nes',
    // icon: <IoIcons.IoIosPaper />,
    cName: 'nav-text'
  },
  {
    title: 'SNES',
    path: '/snes',
    // icon: <FaIcons.FaCartPlus />,
    cName: 'nav-text'
  },
  {
    title: 'GB',
    path: '/gb',
    // icon: <IoIcons.IoMdPeople />,
    cName: 'nav-text'
  },
  {
    title: 'GBC',
    path: '/gbc',
    // icon: <FaIcons.FaEnvelopeOpenText />,
    cName: 'nav-text'
  },
  {
    title: 'GBA',
    path: '/gba',
    // icon: <IoIcons.IoMdHelpCircle />,
    cName: 'nav-text'
  }
];