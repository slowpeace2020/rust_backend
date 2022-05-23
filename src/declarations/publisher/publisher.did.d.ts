import type { Principal } from '@dfinity/principal';
export interface Counter { 'topic' : string, 'value' : bigint }
export interface Subscriber { 'topic' : string }
export interface _SERVICE {
  'publish' : (arg_0: Counter) => Promise<undefined>,
  'subscribe' : (arg_0: Subscriber) => Promise<undefined>,
}
