use std::collections::BTreeMap; // Rust標準ライブラリのB-Tree実装を利用
                               // 実際のDBでは自分でページングと永続化を実装する

/// 簡易的なインデックスをシミュレートする構造体
/// 実際はB+Treeでディスク上に保存される
#[derive(Debug)]
pub struct SimpleDbIndex {
    // Rust標準ライブラリのBTreeMapは、内部でB-Tree構造を実装しています。
    // これをインメモリのインデックスとして利用します。
    // キー: インデックスを張るカラムの値 (例: ID、名前)
    // 値: プライマリキーの値 (実際のデータ行への参照の代わり)
    index_map: BTreeMap<String, String>, // 簡単のため、キーも値もStringとする
}

impl SimpleDbIndex {
    /// 新しいインデックスを作成
    pub fn new() -> Self {
        SimpleDbIndex {
            index_map: BTreeMap::new(),
        }
    }

    /// インデックスにエントリを挿入
    /// `indexed_value`: インデックスを張るカラムの値
    /// `primary_key`: その行のプライマリキーの値 (データへのポインタの代わり)
    pub fn insert(&mut self, indexed_value: String, primary_key: String) {
        self.index_map.insert(indexed_value.clone(), primary_key);
        println!("インデックスに挿入: Key='{}' -> PK='{}'", indexed_value, self.index_map.get(&indexed_value).unwrap());
    }

    /// インデックスを使ってプライマリキーを検索
    /// `indexed_value`: 検索したいインデックスカラムの値
    /// 戻り値: 見つかったプライマリキーのOption
    pub fn find_primary_key(&self, indexed_value: &str) -> Option<&String> {
        let pk = self.index_map.get(indexed_value);
        if pk.is_some() {
            println!("インデックスで検索: Key='{}' -> PK='{}'", indexed_value, pk.unwrap());
        } else {
            println!("インデックスで検索: Key='{}' -> 見つかりませんでした", indexed_value);
        }
        pk
    }

    /// インデックスを使って範囲検索 (概念のみ)
    /// 実際のB+Treeではリーフノードの連結リストを使って高速に走査します。
    pub fn find_range(&self, start_key: &str, end_key: &str) -> Vec<(&String, &String)> {
        let mut results = Vec::new();
        println!("インデックスで範囲検索: '{}' から '{}'", start_key, end_key);
        for (key, pk) in self.index_map.range(start_key.to_string()..=end_key.to_string()) {
            results.push((key, pk));
            println!("  - 見つかった: Key='{}', PK='{}'", key, pk);
        }
        results
    }
}

// ------------------------------------------------------------------------------------

/// 実際のテーブルデータをシミュレートする構造体
/// プライマリキーでアクセスできると仮定
#[derive(Debug, Clone)]
pub struct SimpleDbTable {
    // 実際はディスク上に保存され、プライマリキーで効率的にアクセスできる
    // 簡単のため、HashMapでインメモリにデータを保持
    data: BTreeMap<String, String>, // プライマリキー -> データ行の内容
}

impl SimpleDbTable {
    pub fn new() -> Self {
        SimpleDbTable {
            data: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, primary_key: String, row_data: String) {
        self.data.insert(primary_key, row_data);
    }

    /// プライマリキーを使って行データを取得 (クラスタードインデックスをシミュレート)
    pub fn get_row(&self, primary_key: &str) -> Option<&String> {
        let row = self.data.get(primary_key);
        if row.is_some() {
            println!("テーブルから取得: PK='{}' -> Data='{}'", primary_key, row.unwrap());
        } else {
            println!("テーブルから取得: PK='{}' -> 見つかりませんでした", primary_key);
        }
        row
    }
}

// ------------------------------------------------------------------------------------

pub fn run() {
    println!("--- データベースのシミュレーション開始 ---");

    let mut table = SimpleDbTable::new();
    let mut index_on_name = SimpleDbIndex::new(); // 'name'カラムに対するセカンダリインデックス

    // データの挿入
    println!("\n--- データ挿入 ---");
    table.insert("101".to_string(), "Alice, age 30".to_string());
    index_on_name.insert("Alice".to_string(), "101".to_string());

    table.insert("102".to_string(), "Bob, age 25".to_string());
    index_on_name.insert("Bob".to_string(), "102".to_string());

    table.insert("103".to_string(), "Charlie, age 35".to_string());
    index_on_name.insert("Charlie".to_string(), "103".to_string());

    table.insert("104".to_string(), "David, age 40".to_string());
    index_on_name.insert("David".to_string(), "104".to_string());

    // --- セカンダリインデックスを使った検索のシミュレーション ---
    println!("\n--- 'name'で検索 (セカンダリインデックスを使用) ---");

    let search_name = "Bob";
    println!("\nクエリ: SELECT * FROM users WHERE name = '{}'", search_name);

    // 1. セカンダリインデックスを使ってプライマリキーを検索
    if let Some(pk_from_index) = index_on_name.find_primary_key(search_name) {
        // 2. 取得したプライマリキーを使ってテーブル（クラスタードインデックス）から実際の行データを取得
        table.get_row(pk_from_index);
    } else {
        println!("データが見つかりませんでした。");
    }

    let search_name_not_found = "Eve";
    println!("\nクエリ: SELECT * FROM users WHERE name = '{}'", search_name_not_found);
    if let Some(pk_from_index) = index_on_name.find_primary_key(search_name_not_found) {
        table.get_row(pk_from_index);
    } else {
        println!("データが見つかりませんでした。");
    }

    // --- 範囲検索のシミュレーション ---
    println!("\n--- 'name'で範囲検索 (セカンダリインデックスを使用) ---");
    println!("\nクエリ: SELECT * FROM users WHERE name BETWEEN 'B' AND 'C'");
    let range_results = index_on_name.find_range("B", "C");
    for (_name, pk) in range_results {
        table.get_row(pk); // 各PKを使ってテーブルからデータ取得
    }

    println!("\n--- データベースのシミュレーション終了 ---");
}