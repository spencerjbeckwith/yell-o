import type { Voice, VoiceCategories } from "../types";
import { useState, useEffect } from "react";
import { CaretDownIcon } from "@radix-ui/react-icons";
import css from "./VoiceSelector.module.css";
import classNames from "classnames";
import Spinner from "./Spinner";

function CategorySelect(props: {
    name: VoiceCategories | null;
    currentCategory: VoiceCategories | null;
    setCategory: (category: VoiceCategories | null) => void;
    children: React.ReactNode;
}) {
    const {name, currentCategory, setCategory, children} = props;
    return (
        <label htmlFor={name ?? "nocategory"} className={css.category}>
            <input
                    type="radio"
                    name="category"
                    id={name ?? "nocategory"}
                    checked={currentCategory === (name ?? null)}
                    onChange={(e) => {
                        if (e.target.checked) {
                            setCategory(name);
                        }
                    }}
            />
            { children }
        </label>
    );
}

export default function VoiceSelector(props: {
    voice: Voice | null;
    setVoice: (voice: Voice | null) => void;
}) {
    const {voice, setVoice} = props;
    const [collapsed, setCollapsed] = useState(true);
    const [search, setSearch] = useState("");
    const [category, setCategory] = useState<VoiceCategories | null>("premade");
    const [isLoading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [voices, setVoices] = useState<Voice[] | null>(null);

    useEffect(() => {
        const abort = new AbortController();
        const fetchData = async () => {
            setLoading(true);
            setError(null);
            setVoices(null);
            try {
                const params = new URLSearchParams();
                if (search) {
                    params.set("search", search);
                }
                if (category) {
                    params.set("category", category);
                }
                const response = await fetch(`${window.location.protocol}//${window.location.hostname}:5000/voices?${params.toString()}`, {
                    signal: abort.signal,
                });
                const result = await response.json();
                if (!response.ok) {
                    setError(`Unable to load voices: ${result.error}`);
                }
                setVoices(result.voices);
            } catch (err: unknown) {
                console.error(err);
                if ((err as Error).name !== "AbortError") {
                    setError("An error has occured!");
                }
            } finally {
                setLoading(false);
            }
        };
        fetchData();
        return () => {
            abort.abort();
        };
    }, [search, category]);

    useEffect(() => {
        if (voices && voices.length > 0 && !voice) {
            setVoice(voices[0]);
        }
    }, [voices, voice, setVoice]);

    return (
        <div className={classNames({
            [css.outer]: true,
            [css.open]: !collapsed,
        })}>
            <div className={css.header}>
                <h2>Select Voice</h2>
                <CaretDownIcon className={css.caret} width="2em" height="2em" onClick={() => setCollapsed(!collapsed)}/>
            </div>
            { !collapsed && <>
                <input className={css.search} type="text" placeholder="Search..." onChange={(e) => {
                    setSearch(e.target.value);
                }}/>
                
                <CategorySelect
                    name={null}
                    currentCategory={category}
                    setCategory={setCategory}
                >No Category</CategorySelect>

                <CategorySelect
                    name="premade"
                    currentCategory={category}
                    setCategory={setCategory}
                >Premade Voices</CategorySelect>

                <CategorySelect
                    name="generated"
                    currentCategory={category}
                    setCategory={setCategory}
                >Generated Voices</CategorySelect>

                <CategorySelect
                    name="cloned"
                    currentCategory={category}
                    setCategory={setCategory}
                >Cloned Voices</CategorySelect>

                <CategorySelect
                    name="professional"
                    currentCategory={category}
                    setCategory={setCategory}
                >Professional Voices</CategorySelect>

                { isLoading && <Spinner/> }

                { voices ? <div className={css.voices}>
                    {voices.map((voice, index) => (
                        <p key={index}>{ voice.name }</p>
                    ))}
                </div> : <>
                    {error && <p className={css.error}><strong>{ error }</strong></p>}
                </>}
                { voices && voices.length === 0 && <p className={css.novoices}><i>No voices found.</i></p>}
            </>}
        </div>
    );
}