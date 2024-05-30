import React, { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { invoke } from "@tauri-apps/api/tauri";

export default function CardEdit() {
  const { id } = useParams();
  const [QA, setQA] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [formState, setFormState] = useState({});

  useEffect(() => {
    async function fetchQA() {
      try {
        const QandA = await invoke("questions_and_answer", { fileName: id });
        setQA(QandA);
        setFormState(QandA);
      } catch (error) {
        console.error(`Failed to fetch card detail: ${error}`);
        setError(error);
      } finally {
        setLoading(false);
      }
    }

    fetchQA();
  }, [id]);

  const handleInputChange = (event, key, field) => {
    const { value } = event.target;
    setFormState((prev) => ({
      ...prev,
      [key]: {
        ...prev[key],
        [field]: value,
      },
    }));
  };

  const handleSubmit = async (event) => {
    event.preventDefault();
    try {
      await invoke("save_questions_and_answer", {
        fileName: id,
        data: formState,
      });
      alert("Saved successfully!");
    } catch (error) {
      console.error(`Failed to save card detail: ${error}`);
      setError(error);
    }
  };

  if (loading) {
    return <div>Loading...</div>;
  }

  if (error) {
    return <div>Error loading card details: {error.message}</div>;
  }

  if (!QA) {
    return <div>No details available</div>;
  }

  return (
    <div>
      {/* <form onSubmit={handleSubmit}> */}
        {Object.entries(formState).map(([key, value]) => (
          <div key={key} className={`QA-${key}`}>
            <label htmlFor="">Question {key}</label>
            <input
              type="text"
              name={`question-${key}`}
              value={value["question"]}
              onChange={(e) => handleInputChange(e, key, "question")}
            />
            <label htmlFor="">Answer {key}</label>
            <input
              type="text"
              name={`answer-${key}`}
              value={value["answer"]}
              onChange={(e) => handleInputChange(e, key, "answer")}
            />
            <button type="submit">Update</button>
          </div>
        ))}
      {/* </form> */}
    </div>
  );
}
