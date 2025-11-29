import json
import os
from client import MeilisearchIndexSetup

def main():
    # Meilisearchセットアップインスタンスを作成
    setup: MeilisearchIndexSetup = MeilisearchIndexSetup()

    if not setup.health_check():
        print("Meilisearchサーバーが正常に動作していません")
        return
    
    setup.delete_all_indexes()

    # インデックス設定ファイルを読み込む
    indexes_file: str = "indexes.json"
    if not os.path.exists(indexes_file):
        print(f"インデックス設定ファイルが見つかりません: {indexes_file}")
        return
    with open(indexes_file, "r") as f:
        indexes_config = json.load(f)
    if not isinstance(indexes_config, list):
        print("インデックス設定ファイルの形式が正しくありません")
        return
    for index_config in indexes_config:
        index_name = index_config.get("index_name")
        pid = index_config.get("pid")
        setting_file = index_config.get("setting_file")
        
        if not index_name or not setting_file or not pid:
            print(f"インデックス設定に必要な情報が不足しています: {index_config}")
            continue
        
        # インデックスを作成
        if setup.create_index(index_name, primary_key=pid):
            # 設定ファイルを読み込む
            settings_path = os.path.join("settings", setting_file)
            if not os.path.exists(settings_path):
                print(f"設定ファイルが見つかりません: {settings_path}")
                continue
            
            with open(settings_path, "r") as f:
                settings = json.load(f)
            
            # インデックスの設定を更新
            setup.update_config(index_name, settings)

if __name__ == "__main__":
    main()