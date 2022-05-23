import type { Principal } from '@dfinity/principal';
export interface Post {
  'id' : bigint,
  'text' : string,
  'user_self_id' : string,
  'timestamp' : bigint,
  'user_other_id' : string,
}
export interface _SERVICE {
  'wall' : (arg_0: string) => Promise<Array<Post>>,
  'write' : (arg_0: string, arg_1: string) => Promise<undefined>,
}
