import meilisearch
import meilisearch.errors
import os
from typing import Dict, List, Optional, Any


class MeilisearchIndexSetup:
    def __init__(self):
        """
        Meilisearchクライアントを初期化

        Args:
            host: Meilisearchサーバーのホスト
            api_key: APIキー（必要に応じて）
        """
        host: str = os.environ.get("MEILI_CONNECTION_ADDR", "http://localhost:7700")
#        host: str = "http://localhost:7700"
        api_key: Optional[str] = os.environ.get("MEILI_MASTER_KEY", None)
        self.client = meilisearch.Client(host, api_key)

    def health_check(self) -> bool:
        """
        Meilisearchサーバーのヘルスチェック

        Returns:
            サーバーが正常に動作している場合はTrue
        """
        try:
            health = self.client.health()
            print(f"Meilisearchサーバーの状態: {health}")
            return True
        except Exception as e:
            print(f"ヘルスチェックエラー: {e}")
            return False

    def create_index(self, index_name: str, primary_key: Optional[str] = None) -> bool:
        """
        インデックスを作成

        Args:
            index_name: インデックス名
            primary_key: プライマリキー

        Returns:
            作成成功時True
        """
        try:
            # インデックスが既に存在するかチェック
            try:
                existing_index = self.client.get_index(index_name)
                print(f"インデックス '{index_name}' は既に存在します")
                return True
            except meilisearch.errors.MeilisearchError:
                # インデックスが存在しない場合は作成
                pass

            # インデックス作成
            task = self.client.create_index(index_name, {'primaryKey': primary_key})
            print(f"インデックス '{index_name}' を作成中... (Task ID: {task.task_uid})")

            # タスクの完了を待つ
            self.client.wait_for_task(task.task_uid)
            print(f"インデックス '{index_name}' の作成が完了しました")
            return True

        except Exception as e:
            print(f"インデックス作成エラー: {e}")
            return False

    def update_config(self, index_name: str, config: Dict) -> bool:
        """
        インデックスの設定を更新

        Args:
            index_name: インデックス名
            config: 設定の辞書

        Returns:
            更新成功時True
        """
        try:
            index = self.client.get_index(index_name)
            task = index.update_settings(config)
            self.client.wait_for_task(task.task_uid)
            print(f"インデックス '{index_name}' の設定を更新しました")
            return True
        except Exception as e:
            print(f"設定更新エラー: {e}")
            return False

    def configure_searchable_attributes(self, index_name: str, attributes: List[str]) -> bool:
        """
        検索可能な属性を設定

        Args:
            index_name: インデックス名
            attributes: 検索可能な属性のリスト
        """
        try:
            index = self.client.get_index(index_name)
            task = index.update_searchable_attributes(attributes)
            self.client.wait_for_task(task.task_uid)
            print(f"検索可能属性を設定: {attributes}")
            return True
        except Exception as e:
            print(f"検索可能属性設定エラー: {e}")
            return False

    def configure_displayed_attributes(self, index_name: str, attributes: List[str]) -> bool:
        """
        表示される属性を設定

        Args:
            index_name: インデックス名
            attributes: 表示される属性のリスト
        """
        try:
            index = self.client.get_index(index_name)
            task = index.update_displayed_attributes(attributes)
            self.client.wait_for_task(task.task_uid)
            print(f"表示属性を設定: {attributes}")
            return True
        except Exception as e:
            print(f"表示属性設定エラー: {e}")
            return False

    def configure_filterable_attributes(self, index_name: str, attributes: List[str]) -> bool:
        """
        フィルタリング可能な属性を設定

        Args:
            index_name: インデックス名
            attributes: フィルタリング可能な属性のリスト
        """
        try:
            index = self.client.get_index(index_name)
            task = index.update_filterable_attributes(attributes)
            self.client.wait_for_task(task.task_uid)
            print(f"フィルタリング可能属性を設定: {attributes}")
            return True
        except Exception as e:
            print(f"フィルタリング可能属性設定エラー: {e}")
            return False

    def configure_sortable_attributes(self, index_name: str, attributes: List[str]) -> bool:
        """
        ソート可能な属性を設定

        Args:
            index_name: インデックス名
            attributes: ソート可能な属性のリスト
        """
        try:
            index = self.client.get_index(index_name)
            task = index.update_sortable_attributes(attributes)
            self.client.wait_for_task(task.task_uid)
            print(f"ソート可能属性を設定: {attributes}")
            return True
        except Exception as e:
            print(f"ソート可能属性設定エラー: {e}")
            return False

    def configure_ranking_rules(self, index_name: str, rules: List[str]) -> bool:
        """
        ランキングルールを設定

        Args:
            index_name: インデックス名
            rules: ランキングルールのリスト
        """
        try:
            index = self.client.get_index(index_name)
            task = index.update_ranking_rules(rules)
            self.client.wait_for_task(task.task_uid)
            print(f"ランキングルールを設定: {rules}")
            return True
        except Exception as e:
            print(f"ランキングルール設定エラー: {e}")
            return False

    def configure_stop_words(self, index_name: str, stop_words: List[str]) -> bool:
        """
        ストップワードを設定

        Args:
            index_name: インデックス名
            stop_words: ストップワードのリスト
        """
        try:
            index = self.client.get_index(index_name)
            task = index.update_stop_words(stop_words)
            self.client.wait_for_task(task.task_uid)
            print(f"ストップワードを設定: {stop_words}")
            return True
        except Exception as e:
            print(f"ストップワード設定エラー: {e}")
            return False

    def configure_synonyms(self, index_name: str, synonyms: Dict[str, List[str]]) -> bool:
        """
        同義語を設定

        Args:
            index_name: インデックス名
            synonyms: 同義語の辞書
        """
        try:
            index = self.client.get_index(index_name)
            task = index.update_synonyms(synonyms)
            self.client.wait_for_task(task.task_uid)
            print(f"同義語を設定: {synonyms}")
            return True
        except Exception as e:
            print(f"同義語設定エラー: {e}")
            return False

    def add_sample_documents(self, index_name: str, documents: List[Dict]) -> bool:
        """
        サンプルドキュメントを追加

        Args:
            index_name: インデックス名
            documents: ドキュメントのリスト
        """
        try:
            index = self.client.get_index(index_name)
            task = index.add_documents(documents)
            self.client.wait_for_task(task.task_uid)
            print(f"{len(documents)}件のドキュメントを追加しました")
            return True
        except Exception as e:
            print(f"ドキュメント追加エラー: {e}")
            return False

    def get_index_stats(self, index_name: str) -> Dict[str, Any]:
        """
        インデックスの統計情報を取得

        Args:
            index_name: インデックス名

        Returns:
            統計情報の辞書
        """
        try:
            index = self.client.get_index(index_name)
            stats = index
            return stats.__dict__
        except Exception as e:
            print(f"統計情報取得エラー: {e}")
            return {}

    def delete_all_indexes(self) -> bool:
        """
        全てのインデックスを削除

        Returns:
            削除成功時True
        """
        try:
            indexes = self.client.get_indexes()
            for index  in indexes["results"]:
                task = self.client.delete_index(index.uid)
                self.client.wait_for_task(task.task_uid)
                print(f"インデックス '{index.uid}' を削除しました")
            return True
        except Exception as e:
            print(f"インデックス削除エラー: {e}")
            return False