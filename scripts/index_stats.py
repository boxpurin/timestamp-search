from sys import argv
from client import MeilisearchIndexSetup

index: str = argv[1]
client = MeilisearchIndexSetup()
if client.health_check():
    stats = client.get_index_stats(index)
    for k, v in stats.items():
        print(k, v)