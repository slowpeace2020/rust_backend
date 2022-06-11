

# Starts the replica, running in the background
dfx start --background

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


###todo

todo: 
[rust img接收测试，前端传入]
[transfer转账记录存储，现在的红包转账用户A/B账户名称记录，后续怎么关联principal-id进行转账]
//motoko nft内容定义
[模块之间相互调用，自定义数据不同模块共享的问题, rust和motoko混杂，数据类型怎么共享，
user_profile = { path = "../user_profile" }]
参考linkedup， 全部改成motoko, 每个用户有一个单独的profile, userid不绑定principal_id

  public shared(msg) func redEnvelopeTransfer(ownerA: Text,ownerB: Text, amount: Nat) : async Text {
     //if (amount%2!=0){
      //    amount = amount - 1;
     //}

     if (amount<1){
      return "amount less than 2, couldn't transfer it".
     }

     let saveToPublicAccount : Text = await transfer(owner,amount);

     if(saveToPublicAccount=="Success"){
        let newtransferRecord : TransferRecord = {
                 itemOwnerA = ownerA;
                 itemOwnerB = ownerB;
                 amount = amount;
           };

           List.push(newtransferRecord,unTransferRecordList);
     }

     return saveToPublicAccount
  }




