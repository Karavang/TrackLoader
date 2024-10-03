import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [url, setName] = useState("");

  async function download() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("download", { url }));
  }

  return (
    <div className="container">
      <h1>Welcome to TrackLoader!</h1>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          download();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a link..."
        />

        <button type="submit">Download</button>
      </form>

      <p>{greetMsg}</p>
    </div>
  );
}

export default App;
