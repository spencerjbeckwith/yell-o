import css from "./Spinner.module.css";
import { UpdateIcon } from "@radix-ui/react-icons";

export default function Spinner() {
    return (
        <div className={css.wrapper}>
            <UpdateIcon width="35px" height="35px" className={css.spinner}/>;
        </div>
    );
}