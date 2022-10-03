import React from "react";
import styles from "./PostItem.module.css";
import {Post as PostModel} from "../../models/post";

export interface Props {
  post: PostModel,
}

export const Post: React.FC<Props> = ({post}) => {
  const {text, author} = post;


  return <div className={styles.post}>
    <div className={styles.authorContainer}>
      <img src={author.profile_pic} className={styles.profilePicture} alt={author.dtag}/>
      <div className={styles.authorDetails}>
        <label>@{author.dtag}</label>
        <label>{author.address}</label>
      </div>
    </div>
    <div className={styles.textContainer}>
      <div
        className={styles.text}
      >
        {text}
      </div>
    </div>
  </div>
}
