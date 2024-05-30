import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

import Welcomepage from "./pages/Welcomepage";

import { createBrowserRouter, RouterProvider } from "react-router-dom";
import Menu from "./layouts/Menu";
import Cards from "./pages/Cards";
import Card from "./pages/Card";
import CardEdit from "./pages/CardEdit";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Welcomepage />,
  },
  {
    path: "/cards",
    element: <Cards />,
  },
  {
    path: "/cards/:id",
    element: <Card />
  },
  {
    path: "/cards/:id/edit",
    element: <CardEdit />,
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
