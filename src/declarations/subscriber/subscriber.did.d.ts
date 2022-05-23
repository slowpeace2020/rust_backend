import type { Principal } from '@dfinity/principal';
export type Counter = { 'topic' : string } |
  { 'value' : bigint };
export type Subscriber = { 'topic' : string };
export interface _SERVICE {
  'get_count' : () => Promise<bigint>,
  'setup_subscribe' : (arg_0: Principal, arg_1: string) => Promise<undefined>,
  'update_count' : (arg_0: Counter) => Promise<undefined>,
}
