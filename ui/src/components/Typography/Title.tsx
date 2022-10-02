import React, {CSSProperties} from "react";
import styles from "./Typography.module.css";

export interface Props {
    text: string
    style?: CSSProperties,
}

export const Title: React.FC<Props> = ({text, style}) => {
    return <h1 className={styles.title} style={style}>{text}</h1>
}
