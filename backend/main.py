
import requests
import json

def main():
    url = "http://localhost:8080/get_next_root"
    params = {
        "prefix": "test",
        "kvs": json.dumps([[{"account": 100}, {"account": 100, "balance": 100}]])
        
    }

    response = requests.get(url=url, params=params)
    if response.status_code == 200:
        print(response.json())
    else:
        print(f"请求失败，状态码: {response.status_code}")
        print(response.request.url)
        print(response.text)
        

if __name__ == "__main__":
    main()