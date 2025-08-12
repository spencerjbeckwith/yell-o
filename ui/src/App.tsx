import VoiceSelector from "./components/VoiceSelector";
import { useState } from "react";
import type { Voice } from "./types";
import css from "./App.module.css";
import VoiceTile from "./components/VoiceTile";
import classNames from "classnames";
import Spinner from "./components/Spinner";
import getUrl from "./getUrl";

export default function App() {
    const [voice, setVoice] = useState<Voice | null>(null);
    const [text, setText] = useState("");
    const [submitting, setSubmitting] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [isDone, setIsDone] = useState(false);
    return (
        <main>
            <h1>yell-o</h1>
            <p className={css.centered}><i>What do you have to say?</i></p>
            <VoiceSelector voice={voice} setVoice={setVoice}/>
            { voice &&
                <div>
                    <div className={css.box}>
                        <VoiceTile voice={voice}/>
                        <textarea placeholder="280 character maximum" value={text} onChange={(e) => setText(e.target.value.slice(0, 280))}></textarea>
                    </div>
                    <div className={css.box}>
                        <button
                            className={classNames({
                                [css.speak]: true,
                                [css.submitting]: submitting,
                            })}
                            onClick={() => {
                                if (text.length === 0) {
                                    setError("Enter something to say first!");
                                    return;
                                }
                                if (!submitting) {
                                    setText("");
                                    setError(null);
                                    setSubmitting(true);
                                    setIsDone(false);
                                    const speak = async () => {
                                        try {
                                            const response = await fetch(`${getUrl()}/speak`, {
                                                method: "POST",
                                                headers: {
                                                    "Accept": "application/json",
                                                    "Content-Type": "application/json",
                                                },
                                                body: JSON.stringify({
                                                    voice_id: voice.voice_id,
                                                    text: text,
                                                }),
                                            });
                                            const result = await response.json();
                                            if (!response.ok) {
                                                throw new Error(result.error);
                                            }
                                            setIsDone(true);
                                            setTimeout(() => {
                                                setIsDone(false);
                                            }, 3000);
                                        } catch (err: unknown) {
                                            console.error(err);
                                            setError(`Unable to speak: ${(err as Error).message}`);
                                        } finally {
                                            setSubmitting(false);
                                        }
                                    };
                                    speak();
                                }
                            }}
                        >{ !submitting ? (isDone ? "Done!" : "Speak!") : <Spinner/> }</button>
                    </div>
                    { error && <p className={css.error}><i>{ error }</i></p> }
                </div>
            }
        </main>
    );
};