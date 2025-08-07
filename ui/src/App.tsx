import VoiceSelector from "./components/VoiceSelector";
import { useState } from "react";
import type { Voice } from "./types";
import css from "./App.module.css";
import VoiceTile from "./components/VoiceTile";

export default function App() {
    const [voice, setVoice] = useState<Voice | null>(null);
    const [text, setText] = useState("");
    return (
        <main>
            <h1>yell-o @ { window.location.hostname}</h1>
            <p className={css.centered}><i>Yell-o? Can you hear me yet???</i></p>
            <VoiceSelector voice={voice} setVoice={setVoice}/>
            { voice &&
                <div className={css.box}>
                    <VoiceTile voice={voice}/>
                    <textarea placeholder="280 character maximum" value={text} onChange={(e) => setText(e.target.value.slice(0, 280))}></textarea>
                </div>
            }
        </main>
    );
};