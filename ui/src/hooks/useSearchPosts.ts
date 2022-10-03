import {useCallback, useEffect, useState} from "react";
import {useRootContext} from "./useRootContext";
import {UnixFS} from "ipfs-unixfs";
import {DAGNode, util as DAGUtil} from 'ipld-dag-pb';
import axios from "axios";
import {Particle} from "../models/particle";
import {parseDesmosUri} from "../utils/desmos-utils";
import {Post} from "../models/post";
import {queryPost} from "../gql/queryPost";
import {useApolloClient} from "@apollo/client";

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
  const [posts, setPosts] = useState<Post []>([]);
  const [currentPage, setCurrentPage] = useState(0);
  const [pageLoading, setPageLoading] = useState(false);
  const [endReached, setEndReached] = useState(false);
  const {cyberJs} = useRootContext();
  const apolloClient = useApolloClient();

  const loadPage = useCallback(async (page: number) => {
    if (query?.length ?? 0 > 0) {

      setPageLoading(true);

      // Compute search keyword hash
      const hash = await getIpfsHash(query!);

      let endReached = false;
      let fetchedPosts: Post[] = [];
      let currentPage = page;

      while (!endReached && fetchedPosts.length < 20) {
        console.log("loading page ", currentPage);

        // Fetch particles connected to the query hash
        const particles: Particle[] = await cyberJs!.search(hash, currentPage, 20)
          .then((result) => result.result)
          .catch((err) => {
            console.error(err);
            return [];
          });

        console.log("Loading ipfs content from particles", particles);
        const ipfsContents: string[] = await Promise.all(particles.map((particle) => {
          const url = `https://cloudflare-ipfs.com/ipfs/${particle.particle}`;
          return axios.get(url, {
            timeout: 2000
          }).then((response) => {
            return response.data.toString();
          }).catch((err) => {
            console.error("Error fetching ipfs", err);
            return "";
          })
        }));
        console.log("ipfsContents", ipfsContents);

        console.log("Loading posts");
        const posts = (await Promise.all(
          ipfsContents.map(parseDesmosUri)
            .filter(p => p !== null)
            .map((postInfo) => {
              console.log("post info", postInfo);
              return queryPost(apolloClient, postInfo!.subspaceId, postInfo!.postId)
            })))
          .filter((post) => post !== undefined) as Post [];

        console.log("fetched posts", posts);


        fetchedPosts.push(...posts);

        if (particles.length < 20) {
          endReached = true;
        }

        currentPage++;
      }

      setPosts((prev) => {
        return [...prev, ...fetchedPosts];
      })

      setEndReached(endReached);

      setCurrentPage(currentPage);

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
