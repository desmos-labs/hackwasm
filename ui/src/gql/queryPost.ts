import {ApolloClient, gql, useQuery} from '@apollo/client';
import {Post} from "../models/post";

const GET_POST = gql`
    query Post($subspaceId: bigint, $postId: bigint) {
        post(where: {subspace: {id: {_eq: $subspaceId}}, id: {_eq: $postId}}) {
            author {
                address
                dtag
                profile_pic
            }
            text
        }
    }
`;

export async function queryPost(client: ApolloClient<any>, subspaceId: number, postId: number): Promise<Post | undefined> {
  const result = await client.query({
    query: GET_POST,
    variables: {
      subspaceId: subspaceId.toString(),
      postId: postId.toString()
    }
  })

  return result.data.post[0];
}
