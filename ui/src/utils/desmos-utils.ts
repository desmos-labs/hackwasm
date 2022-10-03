import {ApolloClient} from "@apollo/client";

export interface PostInfo {
  subspaceId: number,
  postId: number,
}

export function parseDesmosUri(uri: string): PostInfo | null {
  if (uri.indexOf("desmos:") === 0) {
    try {
      const url = new URL(uri);
      // pathname should be //SUBSPACE_ID/POST_ID
      const [subspaceString, postString] = url!.pathname.replace("//", "").split("/");

      const subspaceId = parseInt(subspaceString);
      const postId = parseInt(postString);

      // Ensure that are valid subspaceId and postId
      if (isNaN(subspaceId) || subspaceId === 0 || isNaN(postId) || postId === 0) {
        return null;
      } else {
        return {
          subspaceId: subspaceId,
          postId: postId
        }
      }
    } catch (e) {
      return null
    }
  } else {
    return null;
  }
}

function queryPost(client: ApolloClient<any>, subspaceId: number, postId: number) {

}

