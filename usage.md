

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
