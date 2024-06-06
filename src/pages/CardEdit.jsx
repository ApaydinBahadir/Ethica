import React, { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { invoke } from "@tauri-apps/api/tauri";

export default function CardEdit() {
  const { id } = useParams();
  const [QA, setQA] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [formState, setFormState] = useState({});
  const [addFormState, setAddFormState] = useState({});
  const [pressed, setPressed] = useState(0);

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

  const addHandleInputChange = (event, key, field) => {
    const { value } = event.target;
    setAddFormState((prev) => ({
      ...prev,
      [key]: {
        ...prev[key],
        [field]: value,
      },
    }));
  };

  const handleUpdate = async (key) => {
    try {
      await invoke("update_questions_and_answer", {
        fileName: id,
        data: {
          question: formState[key]?.question,
          answer: formState[key]?.answer,
        },
        numberOfQuestion: parseInt(key),
      });
    } catch (error) {
      console.error(`Failed to update question and answer: ${error}`);
      setError(error);
    }
  };

  const addNewQA = () => {
    const newIndex = pressed + 1; // Adjust to start from 1
    setPressed(newIndex);
    setAddFormState((prev) => ({
      ...prev,
      [newIndex]: { question: "", answer: "" },
    }));
  };

  const printData = async () => {
    const sortedAddFormState = Object.fromEntries(
      Object.entries(addFormState).sort(
        ([keyA], [keyB]) => parseInt(keyA) - parseInt(keyB)
      )
    );
    try {
      await invoke("add_questions_and_answer", {
        fileName: id,
        data: sortedAddFormState,
      });
    } catch (error) {
      console.error(`Failed to update question and answer: ${error}`);
      setError(error);
    }
    window.location.reload();
  };

  const removeQA = async (key) => {
    try {
      await invoke("remove_question_answer", {
        filename: id,
        data: {
          question: formState[key]?.question,
          answer: formState[key]?.answer,
        },
      });
    } catch (error) {
      console.error(`Failed to update question and answer: ${error}`);
      setError(error);
    }
    window.location.reload();
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
      {Object.entries(formState).map(([key, value]) => (
        <div key={key} className={`QA-${key}`}>
          <label htmlFor={`question-${key}`}>Question {key}</label>
          <input
            type="text"
            name={`question-${key}`}
            value={value.question}
            id={`question-${key}`}
            onChange={(e) => handleInputChange(e, key, "question")}
          />
          <label htmlFor={`answer-${key}`}>Answer {key}</label>
          <input
            type="text"
            name={`answer-${key}`}
            value={value.answer}
            id={`answer-${key}`}
            onChange={(e) => handleInputChange(e, key, "answer")}
          />
          <button onClick={() => handleUpdate(key)}>Update</button>
          <button onClick={() => removeQA(key)}>Remove</button>
        </div>
      ))}

      <div id="add_qa">
        <button onClick={printData}>Show</button>
        <button onClick={addNewQA}>Add</button>
        {Object.entries(addFormState).map(([key, value]) => (
          <div key={key}>
            <label htmlFor={`new-question-${key}`}>New Question {key}</label>
            <input
              type="text"
              name={`new-question-${key}`}
              value={value.question}
              id={`new-question-${key}`}
              onChange={(e) => addHandleInputChange(e, key, "question")}
            />
            <label htmlFor={`new-answer-${key}`}>New Answer {key}</label>
            <input
              type="text"
              name={`new-answer-${key}`}
              value={value.answer}
              id={`new-answer-${key}`}
              onChange={(e) => addHandleInputChange(e, key, "answer")}
            />
          </div>
        ))}
      </div>
    </div>
  );
}
