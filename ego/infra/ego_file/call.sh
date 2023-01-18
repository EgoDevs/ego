dfx canister call ego_file is_manager '(principal "gpnv5-a22hm-655yn-uqdyc-nux3k-76clz-t46di-6cz3j-ksayy-ym3tg-qae")'
# 0. managers
dfx canister call qoctq-giaaa-aaaaa-aaaea-cai add_managers '(record {managers = vec {principal "qaa6y-5yaaa-aaaaa-aaafa-cai"}})'
dfx canister call ego_bucket remove_managers '(record {managers = vec {principal "gpnv5-a22hm-655yn-uqdyc-nux3k-76clz-t46di-6cz3j-ksayy-ym3tg-qae"}})'

# 3. upload file aaa bbb
dfx canister call ego_file set_file \
'(record {fid = "aaa"; appid = "01"; data = vec {1; 2; 3; 4; 5; 6}; hash="6ac1e56bc78f031059be7be854522c4c"})'
dfx canister call ego_file set_file \
'(record {fid = "bbb"; appid = "01"; data = vec {1; 2; 3; 4; 5; 6}; hash="6ac1e56bc78f031059be7be854522c4c"})'
dfx canister call ego_file set_file \
'(record {fid = "ccc"; appid = "01"; data = vec {1; 2; 3; 4; 5}; hash="6ac1e56bc78f031059be7be854522c4c"})'


dfx canister call ego_file get_file '(record {fid = "aaa"})'
dfx canister call ego_file get_file_info '(record {fid = "aaa"})'
dfx canister call ego_file set_file_stable '(record {fid = "aaa"})'

dfx canister call ego_file list_file
dfx canister call ego_file file_count

