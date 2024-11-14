
import requests
import json

url = "http://localhost:8080/";

key1 = {"user_id": "1000"}
value1 = {
            "nonce": 1,
            "balance": 10000000000000
        }

key2 = {"user_id": 1001}
value2 = {
            "nonce": 1,
            "balance": 10000000000001
        }

prefix = "test"


def update_key1(path: str):
    j1 = {
        "key": key1,
        "prefix": prefix,
        "value": value1,
    }
    
    print("key1 更新数据后root是: \n", post(path, j1))

def update_key2(path: str):
    j2 = {
        "key": key2,
        "prefix": prefix,
        "value": value2,
    }

    print("key2 更新数据后root是:\n", post(path, j2))
    

def get_merkel_proof(path: str):
    j = {
        "key": key1,
        "prefix": prefix
    }
    p = post(path, j)
    print("key1的默克尔证明是: \n", p)
    return p

def get_next_root(path: str):
    j = {
        "prefix": prefix,
        "kv": {
            "key": key2,
            "value": value2
        }
        
    }
    next_root = post(path, j);
    print("预计key2更新后root值是: \n", next_root)
    return next_root



def get_value(path: str):
    j = {
        "key": key1,
        "prefix": prefix
    }
    print("key1的叶子值是: \n", post(path, j))

def post(path: str, j: dict):
    r = url + path
    response = requests.post(url=r, json=j)
    if response.status_code == 200:
        pretty_json = json.dumps(response.json(), indent=4)
        return pretty_json
    else:
        return f"失败状态： {response.status_code}, 失败信息： {response.text}。"

def verify(path: str, proof: dict):
    print(f"key1验证结果是: \n {post(path, proof)}")
    

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