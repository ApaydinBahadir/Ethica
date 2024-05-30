import React, { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { invoke } from "@tauri-apps/api/tauri";

export default function Card() {
  const { id } = useParams();
  const [cardDetail, setCardDetail] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    async function fetchCardDetail() {
      try {
        const detail = await invoke("card_details", { fileName: id });
        setCardDetail(detail);
        console.log(typeof detail);
      } catch (error) {
        console.error(`Failed to fetch card detail: ${error}`);
        setError(error);
      } finally {
        setLoading(false);
      }
    }

    fetchCardDetail();
  }, [id]);

  if (loading) {
    return <div>Loading...</div>;
  }

  if (error) {
    return <div>Error loading card details: {error.message}</div>;
  }

  if (!cardDetail) {
    return <div>No details available</div>;
  }

  return (
    <div id="details">
      <div id="card_detail">{cardDetail["Count Of Questions"]}</div>
    </div>
  );
}
