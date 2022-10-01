import React from "react";
import styles from "./PostItem.module.css";

export interface Props {
  dtag: string,
  author: string,
  profilePicture: string,
  text: string,
}

export const Post: React.FC<Props> = ({dtag, author, profilePicture, text}) => {
  return <div className={styles.post}>
    <div className={styles.authorContainer}>
      <img src={profilePicture} className={styles.profilePicture} alt={dtag}/>
      <div className={styles.authorDetails}>
        <label>{dtag}</label>
        <label>{author}</label>
      </div>
    </div>
    <div
      className={styles.text}
    >
      {text}
    </div>
  </div>
}
