import { useState } from "react";
import { writeFile, BaseDirectory, mkdir } from "@tauri-apps/plugin-fs";

function App() {
  const [recorderState, setRecorderState] = useState<
    { type: "recording"; onStop: () => void } | { type: "standby" }
  >({ type: "standby" });

  async function handleRecord() {
    const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
    const mediaRecorder = new MediaRecorder(stream);
    const chunks: Blob[] = [];

    mediaRecorder.ondataavailable = (e) => {
      chunks.push(e.data);
    };

    mediaRecorder.onstop = async () => {
      stream.getTracks().forEach((track) => track.stop());
      const blob = new Blob(chunks, { type: "audio/ogg; codecs=opus" });
      await mkdir("recordings", {
        recursive: true,
        baseDir: BaseDirectory.AppData,
      });
      const filename = `${new Date().valueOf()}.ogg`;
      await writeFile(
        `recordings/${filename}`,
        new Uint8Array(await blob.arrayBuffer()),
        {
          baseDir: BaseDirectory.AppData,
        },
      );
    };

    setRecorderState({
      type: "recording",
      onStop: () => {
        mediaRecorder.stop();
        setRecorderState({ type: "standby" });
      },
    });
    mediaRecorder.start();
  }

  return (
    <div className="App">
      <button
        onClick={
          recorderState.type === "recording"
            ? recorderState.onStop
            : handleRecord
        }
      >
        {recorderState.type === "recording" ? "Stop" : "Record"}
      </button>
    </div>
  );
}

export default App;
