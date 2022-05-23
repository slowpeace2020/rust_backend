export const idlFactory = ({ IDL }) => {
  const Counter = IDL.Record({ 'topic' : IDL.Text, 'value' : IDL.Nat64 });
  const Subscriber = IDL.Record({ 'topic' : IDL.Text });
  return IDL.Service({
    'publish' : IDL.Func([Counter], [], []),
    'subscribe' : IDL.Func([Subscriber], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
