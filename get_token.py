import urllib.request, json
url = 'https://gamma-api.polymarket.com/events?closed=false&active=true&limit=1'
req = urllib.request.Request(url, headers={'User-Agent': 'Mozilla/5.0'})
try:
    with urllib.request.urlopen(req) as response:
        data = json.loads(response.read().decode())
        market = data[0]['markets'][0]
        token_str = market.get('clobTokenIds', '[]')
        tokens = json.loads(token_str)
        asset_id = tokens[0] if len(tokens) > 0 else 'NO_CLOB_TOKENS'
        print(f'ASSET_ID: {asset_id}')
except Exception as e:
    pass
