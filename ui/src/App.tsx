import VoiceSelector from "./components/VoiceSelector";
import { useState } from "react";
import type { Voice } from "./types";
import css from "./App.module.css";

export default function App() {
    const [voice, setVoice] = useState<Voice | null>(null)
    return (
        <main>
            <h1 className={css.header}>yell-o</h1>
            <VoiceSelector voice={voice} setVoice={setVoice}/>
            { voice ? <p>Current voice: {voice.name}</p> : <p><i>No voice selected!</i></p>}
        </main>
    );
};