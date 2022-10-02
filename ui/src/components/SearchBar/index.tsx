import React, {useCallback} from "react";
import styles from "./SearchBar.module.css"
import {AiOutlineSearch} from "react-icons/ai"

export interface Props {
    // Function called when the user is typing.
    onValueChange?: (text: string) => void,
    // Function called when the user press enter.
    onSubmit?: (text: string) => void,
    // Default input value.
    defaultValue?: string
}

export const SearchBar: React.FC<Props> = ({onValueChange, onSubmit, defaultValue}) => {

    const onInputChange = useCallback((input: React.ChangeEvent<HTMLInputElement>) => {
        const text = input.target.value;
        if (onValueChange) {
            onValueChange(text);
        }
    }, [onValueChange]);

    const onInputKeyDown = useCallback((event: React.KeyboardEvent<HTMLInputElement>) => {
        const text = event.currentTarget.value;
        if (onSubmit && event.key === "Enter") {
            onSubmit(text);
        }
    }, [onSubmit])


    return <div className={styles.container}>
        <AiOutlineSearch className={styles.icon}/>
        <input type="text" inputMode="text" className={styles.input}
               placeholder="Search post"
               multiple={false}
               defaultValue={defaultValue}
               onChange={onInputChange}
               onKeyDown={onInputKeyDown}
        />
    </div>
}
