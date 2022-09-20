

# Starts the replica, running in the background
dfx start --background

#清空canister
dfx start --background --clean

# Deploys your canisters to the replica and generates your candid interface
dfx deploy

##命令行调用后端服务,rust_counter为模块名称，
###get为其中的一个方法，方法名可以在did文件的service数组元素中找到：

dfx canister call rust_counter get

#带参数 
###location为其中的一个方法，"San Francisco"为字符串参数：
dfx canister call location_hello location "San Francisco"

#带多个参数
dfx canister call contacts insert '("Amy Lu","01 916-335-2042")'

dfx canister call hotel guestroom '("Deluxe Suite",42,true)'


##查看某个模块的canister id
dfx canister id projectname 

##查看自己的principal-id
dfx identity get-principal

##查看本地所有用户
dfx identity list

##查看可能的其他命令
dfx identity help

##以某个用户的身份调用接口方法
dfx --identity username call module_name method_name parameters

##给某个principal id的用户转账
dfx canister call token transfer '(principal "4p27i-ym6ti-34gws-3evnd-k6hdp-5iv6a-g7hzw-nc33q-lpyqi-elhkz-tae",10000)'
dfx canister call token transfer '(principal "2vxsx-fae",100000)'

#停止服务
dfx stop

##message json数据格式
###条件查询
"
{
            \"user_self_id\": \"2vxsx-fae\",
            \"user_other_id\": \"Menlo Park, CA\"
  }
"

"
{
            \"user_self_id\": \"2vxsx-fae\"
              }
"

###查询全部数据
"{}"

##获取当前用户信息
dfx canister call user_profile getOwnProfile

##principalID通过用户名关联一个用户资料
linkPrincipalID
dfx canister call user_profile link_principal_id "username"

##dfx deploy 初始化参数
dfx deploy --argument "(record{
acounts=vec{
record{owner=principal \"test1\";tokens=\"test2\"},
record{owner=principal \"test1\";tokens=\"test2\"}
};
proposals=vec{};
})"

$ dfx canister --network=ic --no-wallet call aanaa-xaaaa-aaaah-aaeiq-cai wallet_create_canister "(record {user_other_id=\"testuser\"; text=\"gogogo\"; })"
$ dfx canister call aanaa-xaaaa-aaaah-aaeiq-cai wallet_create_canister "(record {cycles= (AMOUNT:nat64); controller= (null); })"


(
  variant {
    17_724 = record { 1_313_628_723 = principal "CREATED_CANISTER_ID" }
  },
)

###todo

todo: 
[rust img接收测试，前端传入]
[transfer转账记录存储，现在的红包转账用户A/B账户名称记录，后续怎么关联principal-id进行转账]
//motoko nft内容定义
[模块之间相互调用，自定义数据不同模块共享的问题, rust和motoko混杂，数据类型怎么共享，
user_profile = { path = "../user_profile" }]
参考linkedup， 全部改成motoko, 每个用户有一个单独的profile, userid不绑定principal_id


ledger setup
https://internetcomputer.org/docs/current/developer-docs/functionality/ledger/ledger-local-setup/

deploy new token 
https://internetcomputer.org/docs/current/developer-docs/functionality/ledger/deploy-new-token


rust token 账户初始化问题， 命令行参数格式问题

###todo
token读取rust返回数据结构类型

2022/8/20
邀请码放在用户模块还是contract模块
用户B响应邀请之后的流程，完成用户A和B的合约？用户A发起合约的存储

