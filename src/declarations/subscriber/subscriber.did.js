export const idlFactory = ({ IDL }) => {
  const Counter = IDL.Variant({ 'topic' : IDL.Text, 'value' : IDL.Nat64 });
  return IDL.Service({
    'get_count' : IDL.Func([], [IDL.Nat64], []),
    'setup_subscribe' : IDL.Func([IDL.Principal, IDL.Text], [], []),
    'update_count' : IDL.Func([Counter], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
