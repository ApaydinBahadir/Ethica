import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

import Welcomepage from "./pages/Welcomepage";

import { createBrowserRouter, RouterProvider } from "react-router-dom";
import Menu from "./layouts/Menu";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Welcomepage />,
  },
]);

function App() {
  return (
    <div>
      <Menu/>
      <RouterProvider router={router} />
    </div>
  );
}

export default App;
