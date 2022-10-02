import {useCallback, useEffect, useState} from "react";
import {useRootContext} from "./useRootContext";
import {UnixFS} from "ipfs-unixfs";
import { DAGNode, util as DAGUtil } from 'ipld-dag-pb';
import axios from "axios";

function getIpfsHash(string: string): Promise<string> {
  return new Promise(async (resolve, reject) => {
    const strLength = string.length;
    const unixFsFile = new UnixFS({
      type: 'file',
      data: Buffer.from(string)
    });

    // Dirty hack to have the buffer with the right bytes.
    // TODO: Investigate why UnixFS is not creating the right buffer after marshal.
    const buffer = Buffer.concat([new Buffer([10, strLength + 6]), new Buffer(unixFsFile.marshal())]);
    const dagNode = new DAGNode(buffer);
    const cid = await DAGUtil.cid(dagNode.Data, {
      cidVersion: 0,
    });

    resolve(cid.toBaseEncodedString());
  });
}

export function useSearchPosts(query: string | undefined) {
  const [posts, setPosts] = useState<string []>([]);
  const [currentPage, setCurrentPage] = useState(0);
  const [pageLoading, setPageLoading] = useState(false);
  const [endReached, setEndReached] = useState(false);
  const {cyberJs} = useRootContext();

  const loadPage = useCallback(async (page: number) => {
    if (query?.length ?? 0 > 0) {

      console.log("loading page ", page);

      setPageLoading(true);

      const hash = await getIpfsHash(query!);
      console.log("hash", hash);
      const particles = await cyberJs!.search(hash, page, 20)
        .then((result) => result.result)
        .catch((err) => {
          console.error(err);
          return [];
        });

      // @ts-ignore
      const posts = await Promise.all(particles.map((particle) => {
        console.log("particle", particle);
        const url = `https://cloudflare-ipfs.com/ipfs/${particle.particle}`;
        return axios.get(url).then((response) => {
          return response.data.toString()
        })
      }));

      setPosts((prev) => {
        return [...prev, ...posts];
      })

      if (posts.length < 20) {
        setEndReached(true);
      }

      setCurrentPage(page);

      setPageLoading(false)

      console.log("Page loaded ", page);
    }
  }, [query]);

  const loadNextPage = useCallback(() => {
    if (!endReached) {
      return loadPage(currentPage + 1);
    } else {
      return Promise.resolve();
    }
  }, [loadPage, currentPage, endReached])

  // Load the content
  useEffect(() => {
    setPosts([]);
    setEndReached(false);
    loadPage(0)
  }, [loadPage]);

  return {
    posts,
    pageLoading,
    loadNextPage,
    endReached
  }
}
