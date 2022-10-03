export interface Profile {
  address: string,
  dtag: string,
  profile_pic: string,
}

export interface Post {
  author: Profile
  text: string
}
