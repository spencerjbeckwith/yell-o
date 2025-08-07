import css from "./VoiceTile.module.css";
import type { Voice } from "../types";
import classNames from "classnames";

function clean(input: string): string {
    const replaced = input.replaceAll("_", " ");
    return replaced[0].toUpperCase() + replaced.substring(1);
}

export default function VoiceTile(props: {
    voice: Voice;
    onClick?: () => void;
}) {
    const {voice, onClick} = props;
    return (
        <div className={classNames({
            [css.tile]: true,
            [css.clickable]: onClick !== undefined,
        })} onClick={onClick}>
            <h3>{ voice.name }</h3>
            { voice.labels && <p>
                { voice.labels.age && <span className={css.age}>{clean(voice.labels.age)}</span>}
                { voice.labels.description && <span className={css.description}>{clean(voice.labels.description)}</span>}
                { voice.labels.accent && <span className={css.accent}>{clean(voice.labels.accent)}</span>}
                { voice.labels.gender && <span className={css.gender}>{clean(voice.labels.gender)}</span>}
            </p> }
            <p>{ voice.description }</p>
        </div>
    );
}