import React, { useState, useEffect } from "react";
import { invoke } from '@tauri-apps/api/tauri';
import "../styles/Menu.css";

export default function Menu() {
  const [cardCount, setCardCount] = useState(null);

  useEffect(() => {
    async function fetchCardCount() {
      try {
        const count = await invoke('getcards', { name: 'example_card_name' });
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
        {cardCount !== null ? `Card Count: ${cardCount}` : 'Loading...'}
      </div>
    </div>
  );
}
