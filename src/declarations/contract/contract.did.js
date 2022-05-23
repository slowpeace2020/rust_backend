export const idlFactory = ({ IDL }) => {
  const Post = IDL.Record({
    'id' : IDL.Int,
    'text' : IDL.Text,
    'user_self_id' : IDL.Text,
    'timestamp' : IDL.Int,
    'user_other_id' : IDL.Text,
  });
  return IDL.Service({
    'wall' : IDL.Func([IDL.Text], [IDL.Vec(Post)], ['query']),
    'write' : IDL.Func([IDL.Text, IDL.Text], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
