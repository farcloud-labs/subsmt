
import requests
import json

url = "http://localhost:8080/";

user1_address = "1000"
user2_address = "1001"
prefix = "test"
balance = "1000000000000000000"


def update_key1(path: str):
    j1 = {
        "address": user1_address,
        "balance": balance,
        "nonce": 1,
        "prefix": prefix
    }
    
    print("user1 更新数据后root是: \n", post(path, j1))

def update_key2(path: str):
    j2 = {
        "address": user2_address,
        "balance": balance,
        "nonce": 1,
        "prefix": prefix
    }

    print("user2 更新数据后root是:\n", post(path, j2))
    

def get_merkel_proof(path: str):
    j = {
        "address": user1_address,
        "prefix": prefix
    }
    p = post(path, j)
    print("user1 的默克尔证明是: \n", p)
    return p

def get_next_root(path: str):
    j = {
        "address": user2_address,
        "balance": balance,
        "nonce": 1,
        "prefix": prefix
        
    }
    next_root = post(path, j);
    print("预计 user2 更新后root值是: \n", next_root)
    return next_root



def get_value(path: str):
    j = {
        "address": user1_address,
        "prefix": prefix
    }
    print("user1 的叶子值是: \n", post(path, j))

def post(path: str, j: dict):
    r = url + path
    response = requests.post(url=r, json=j)
    if response.status_code == 200:
        pretty_json = json.dumps(response.json(), indent=4)
        return pretty_json
    else:
        return f"失败状态： {response.status_code}, 失败信息： {response.text}。"

def verify(path: str, proof: dict):
    print(f"user1验证结果是: \n {post(path, proof)}")
    

def main():
    update_key1("update")
    print("--"*50)
    get_next_root("next_root")
    print("--"*50)
    update_key2("update")
    print("--"*50)
    p = get_merkel_proof("merkle_proof")
    print("--"*50)
    p = json.loads(p)
    get_value("value")
    print("--"*50)
    verify("verify", p)
    print("--"*50)
    
if __name__ == "__main__":
    main()