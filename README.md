# LICP

## 部署方式

```bash
# 启动 本地容器执行环境
dfx start --background

# 部署 LICP_backend 容器
dfx deploy LICP_backend

# 设置变量
# 铸造角色为 LICP 后端
export MINTROLE="LICP 后端"
# 控制器为当前身份主体
export PRINCIPAL=$(dfx identity get-principal)

# 部署 LICP代币 容器
dfx deploy icrc1-ledger --argument "(record {  token_symbol = \"LICP\";  token_name = \"Stake ICP\";  minting_account = record { owner = principal \"$MINTROLE\"  };  transfer_fee = 0;  metadata = vec {};  initial_balances = vec {};  archive_options = record {    num_blocks_to_archive = 2000;    trigger_threshold = 1000;    controller_id = principal \"$PRINCIPAL\";  };},)"

# 测试部署 ICP 分类账容器
# 创建切换一个新身份
dfx identity new minter  # 创建新身份
dfx identity use minter  # 切换新身份
# 设置新身份为铸造角色
export MINT_ACC=$(dfx ledger account-id)

# 切换回默认身份
dfx identity use default
# 设置初始化时铸造代币的用户
export LEDGER_ACC=$(dfx ledger account-id)
# 设置默认身份为存档罐控制器
export ARCHIVE_CONTROLLER=$(dfx identity get-principal)
# 部署容器
dfx deploy ledger --argument '(record {minting_account = "'${MINT_ACC}'"; initial_values = vec { record { "'${LEDGER_ACC}'"; record { e8s=100_000_000_000 } }; }; send_whitelist = vec {}; archive_options = opt record { trigger_threshold = 2000; num_blocks_to_archive = 1000; controller_id = principal "'${ARCHIVE_CONTROLLER}'" }})'


# 拥有mint身份的用户进行铸造 icp
dfx canister call ledger transfer '(record { to = '$(python3 -c 'print("vec{" + ":nat8;".join([str(b) for b in bytes.fromhex("'$LEDGER_ACC'")]) + ":nat8}")')'; fee = record {e8s=0:nat64}; memo=2:nat64; amount=record {e8s=100000:nat64}; })'
# 查询子账户icp余额
dfx canister call ledger account_balance '(record { account = '$(python3 -c 'print("vec{" + ":nat8;".join([str(b) for b in bytes.fromhex("'$LEDGER_ACC'")]) + ":nat8}")')' })'
# 非mint用户给别人发送 icp
dfx canister call ledger transfer '(record { to = '$(python3 -c 'print("vec{" + ":nat8;".join([str(b) for b in bytes.fromhex("063fddde9406d9e2a25bf98a55e141cdd50fbdea4ae0e57caa6d00d6b5ec4c35")]) + ":nat8}")')'; fee = record {e8s=10000:nat64}; memo=2:nat64; amount=record {e8s=800:nat64}; })'
# 验证目标用户余额
dfx canister call ledger account_balance '(record { account = '$(python3 -c 'print("vec{" + ":nat8;".join([str(b) for b in bytes.fromhex("063fddde9406d9e2a25bf98a55e141cdd50fbdea4ae0e57caa6d00d6b5ec4c35")]) + ":nat8}")')' })'

```

# 使用 didc 验证输入参数

## 关于安装
https://github.com/dfinity/candid/releases <br>

## 使其称为可执行文件

chmod +x xxx文件

## 关于使用

https://github.com/dfinity/candid/tree/master/tools/didc <br>


``` bush
didc encode '(record { to = '$(python3 -c 'print("vec{" + ":nat8;".join([str(b) for b in bytes.fromhex("063fddde9406d9e2a25bf98a55e141cdd50fbdea4ae0e57caa6d00d6b5ec4c35")]) + ":nat8}")')'; fee = record {e8s=0:nat64}; memo=2:nat64; amount=record {e8s=100000:nat64}; })'

4449444c036c04fbca0101c6fcb60202ba89e5c20478d8a38ca80d026d7b6c01e0a9b30278010020063fddde9406d9e2a25bf98a55e141cdd50fbdea4ae0e57caa6d00d6b5ec4c3500000000000000000200000000000000a086010000000000

didc decode 4449444c036c04fbca0101c6fcb60202ba89e5c20478d8a38ca80d026d7b6c01e0a9b30278010020063fddde9406d9e2a25bf98a55e141cdd50fbdea4ae0e57caa6d00d6b5ec4c3500000000000000000200000000000000a086010000000000

(
  record {
    25_979 = blob "\06?\dd\de\94\06\d9\e2\a2[\f9\8aU\e1A\cd\d5\0f\bd\eaJ\e0\e5|\aam\00\d6\b5\ecL5";
    5_094_982 = record { 5_035_232 = 0 : nat64 };
    1_213_809_850 = 2 : nat64;
    3_573_748_184 = record { 5_035_232 = 100_000 : nat64 };
  },
)
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.
