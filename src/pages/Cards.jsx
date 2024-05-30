import React, { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "../styles/Cards.css";

export default function Cards() {
  const [cards, setCards] = useState(null);

  useEffect(() => {
    async function fetchCards() {
      try {
        const cards = await invoke("cards_add_list", {});
        setCards(cards);
      } catch (error) {
        console.error(`Failed to fetch cards: ${error}`);
      }
    }

    fetchCards();
  }, []);

  return (
    <div id="cards">
      {cards &&
        cards.map((element, index) => (
          <a className="card" href={`/cards/${element}`} key={index}>
            {element}
          </a>
        ))}
    </div>
  );
}
