import {useCallback, useEffect, useState} from "react";

function delay(ms: number): Promise<void> {
  return new Promise(resolve => {
    setTimeout(() => {
      resolve();
    }, ms);
  })
}

export function useSearchPosts(query: string | undefined) {
  const [posts, setPosts] = useState<string []>([]);
  const [currentPage, setCurrentPage] = useState(0);
  const [pageLoading, setPageLoading] = useState(false);

  const loadPage = useCallback(async (page: number) => {
    if (query?.length ?? 0 > 0) {

      console.log("loading page ", page);

      setPageLoading(true);

      await delay(500);

      //TODO: Fetch from cyb.ai
      const newElements = Array.from({length: 20}, (x, i) => {
        const index = 20 * page + i;
        return `${query} ${index}`
      });

      // Update the posts
      setPosts((posts) => {
        return [...posts, ...newElements];
      })

      // Update the current page
      setCurrentPage(page);

      setPageLoading(false)

      console.log("Page loaded ", page);
    }
  }, [query]);

  const loadNextPage = useCallback(() => {
    return loadPage(currentPage + 1);
  }, [loadPage, currentPage])

  // Load the content
  useEffect(() => {
    setPosts([]);
    loadPage(0)
  }, [loadPage]);

  return {
    posts,
    pageLoading,
    loadNextPage
  }
}
