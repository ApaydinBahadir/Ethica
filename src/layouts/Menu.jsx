import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "../styles/Menu.css";

export default function Menu() {
  const [cardCount, setCardCount] = useState(null);

  useEffect(() => {
    async function fetchCardCount() {
      try {
        const count = await invoke("card_count", {});
        setCardCount(count);
      } catch (error) {
        console.error(`Failed to fetch card count: ${error}`);
      }
    }

    fetchCardCount();
  }, []);

  return (
    <div id="menu">
      <div id="cardCount">
        <a href="/cards">
          {cardCount !== null ? `Card Count: ${cardCount}` : "Loading..."}
        </a>
      </div>

      <div>
        <a href="/cards/deneme/edit">deneme click</a>
      </div>

      <div>
        <a href="/cards/title/edit">click</a>
      </div>

      <div>
        <a>{window.location.pathname}</a>
      </div>
    </div>
  );
}
