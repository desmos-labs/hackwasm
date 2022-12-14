import React, {useCallback, useMemo} from "react";
import styles from "./PostList.module.css";
import {useSearchPosts} from "../../hooks/useSearchPosts";
import {Post} from "./Post";

export interface Props {
  searchText?: string,
}

export const PostList: React.FC<Props> = ({searchText}) => {
  const {pageLoading, posts, loadNextPage, endReached} = useSearchPosts(searchText)

  const postElements = useMemo(() => {
    return posts.map((post, index) => {
      return <Post
        key={`post-${index}`}
        post={post}
      />
    })
  }, [posts])

  const trackScrolling = useCallback((event: React.UIEvent<HTMLOListElement, UIEvent>) => {
    const target = event.currentTarget;
    const {offsetHeight, scrollTop, scrollHeight} = target;


    if (offsetHeight + scrollTop + 50 >= scrollHeight) {
      if (!pageLoading) {
        loadNextPage()
      }
    }

  }, [pageLoading, loadNextPage])

  return <div className={styles.container}>
    <ol className={styles.list} onScroll={trackScrolling}>
      {postElements}
    </ol>
    {pageLoading && <p>Loading...</p>}
    {!pageLoading && posts.length === 0 && endReached && <p>No elements</p>}
  </div>

}
