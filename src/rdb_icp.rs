use std::collections::BTreeMap;

// =======================================================
// RDBの行構造 (インデックスに含まれるものと含まれないもの)
// =======================================================
#[derive(Debug, Clone)]
struct User {
    id: u32,
    city: String, // インデックスに含まれる
    age: u8,      // インデックスに含まれる
    score: u32,   // インデックスに含まれない（非キーカラム）
}

// 複合インデックス: (City, Age)
type CompositeIndex = BTreeMap<(String, u8), Vec<u32>>;
type UsersTable = Vec<User>;

// =======================================================
// デモデータとインデックスのセットアップ
// =======================================================
fn setup_data() -> (UsersTable, CompositeIndex) {
    let mut table = UsersTable::new();
    let mut index = CompositeIndex::new();
    let cities = vec!["東京", "大阪", "福岡"];

    for i in 0..30 {
        let city = cities[i % cities.len()].to_string();
        let age = (20 + i % 10) as u8;
        let score = 100 - i as u32; // スコアはインデックスに含まれない

        let user = User { id: i as u32 + 1, city: city.clone(), age, score };
        
        let row_index = i as u32;
        table.push(user);

        // インデックス構築: (city, age)
        index.entry((city, age))
             .or_insert_with(Vec::new)
             .push(row_index);
    }
    (table, index)
}

// =======================================================
// 1. ICPなしの検索 (従来の方式)
// =======================================================
// ストレージエンジン側（テーブルアクセス）: インデックスキーのみでフィルタリング
fn classic_storage_lookup(index: &CompositeIndex, table: &UsersTable, city: &str) -> Vec<User> {
    println!("\n--- 1. ICPなし: 従来の検索方式 ---");
    let mut users_to_check = Vec::new();
    let mut index_searches = 0;
    let mut table_accesses = 0;

    // 1-A. インデックスによるレンジスキャン (City="東京"で開始)
    // 注意: ここではCityとAgeの両方をキーとするBTreeMapのイテレータを使用
    let start_key = (city.to_string(), 0);
    let end_key = (city.to_string(), 255); // 年齢の最大値より大きい

    for (key, row_indices) in index.range(start_key..=end_key) {
        if key.0 != city { continue; } // イテレータの範囲外チェック
        index_searches += 1;

        // 1-B. インデックスに該当する全行を取得し、サーバー側へ転送（ここでAgeの条件は無視）
        for &i in row_indices {
            if let Some(user) = table.get(i as usize) {
                users_to_check.push(user.clone());
                table_accesses += 1; // テーブルへのアクセス（データ転送）が発生
            }
        }
    }
    
    println!("  * ストレージ側（インデックス層）の処理結果:");
    println!("    -> インデックスキー検索回数: {}", index_searches);
    println!("    -> **テーブルアクセス（データ転送）回数:** {}", table_accesses);


    // 1-C. サーバー側: 転送されたデータに対して残りの条件 (Age, Score) をフィルタリング
    let mut results = Vec::new();
    let mut final_checks = 0;
    for user in users_to_check {
        final_checks += 1;
        // Ageの条件 (Age < 25) と Scoreの条件 (Score > 80) はサーバー側で評価
        if user.age < 25 && user.score > 80 {
            results.push(user);
        }
    }

    println!("  * サーバー側（最終フィルタリング）の処理結果:");
    println!("    -> チェックした行数: {}", final_checks);
    println!("    -> 最終結果件数: {}", results.len());
    results
}

// =======================================================
// 2. Index Condition Pushdown (ICP) を使用した検索
// =======================================================
// ストレージエンジン側（テーブルアクセス）: インデックスキーとインデックスカラムのフィルタを適用
fn icp_storage_lookup(index: &CompositeIndex, table: &UsersTable, city: &str) -> Vec<User> {
    println!("\n--- 2. ICPあり: Index Condition Pushdown ---");
    let mut users_to_check = Vec::new();
    let mut index_searches = 0;
    let mut table_accesses = 0;

    // 2-A. インデックスによるレンジスキャン (City="東京"で開始)
    let start_key = (city.to_string(), 0);
    let end_key = (city.to_string(), 255);

    for (key, row_indices) in index.range(start_key..=end_key) {
        if key.0 != city { continue; } // Cityのレンジチェック
        index_searches += 1;

        // 2-B. ICP: **インデックスに含まれる Age の条件を、ストレージエンジン側で評価**
        let age_from_index = key.1; 
        if age_from_index >= 25 {
             // インデックスの時点で、Age < 25 の条件を満たさないので、
             // このインデックスキーに対応するデータ行は読み込まない（プッシュダウン成功）
             continue; 
        }

        // 2-C. 条件を満たした行のみ、テーブルにアクセスしサーバー側へ転送
        for &i in row_indices {
            if let Some(user) = table.get(i as usize) {
                users_to_check.push(user.clone());
                table_accesses += 1; // 転送回数が減る！
            }
        }
    }

    println!("  * ストレージ側（インデックス層）の処理結果:");
    println!("    -> インデックスキー検索回数: {}", index_searches);
    println!("    -> **テーブルアクセス（データ転送）回数:** {}", table_accesses);

    // 2-D. サーバー側: 転送されたデータに対して残りの条件 (Score) をフィルタリング
    let mut results = Vec::new();
    let mut final_checks = 0;
    for user in users_to_check {
        final_checks += 1;
        // Ageの条件はストレージ側で処理済み。ここでは Scoreの条件 (Score > 80) のみ評価
        if user.score > 80 {
            results.push(user);
        }
    }

    println!("  * サーバー側（最終フィルタリング）の処理結果:");
    println!("    -> チェックした行数: {}", final_checks);
    println!("    -> 最終結果件数: {}", results.len());
    results
}


// =======================================================
// 実行
// =======================================================
pub fn run() {
    let (users_table, composite_index) = setup_data();
    
    let search_city = "東京";
    // 検索条件: WHERE city = '東京' AND age < 25 AND score > 80
    // インデックス: (city, age)

    println!("=======================================================");
    println!("        索引条件プッシュダウン (ICP) デモ 🚀");
    println!("=======================================================");
    println!("総データ件数: {} 件", users_table.len());
    println!("検索条件: City='{}' AND Age < 25 AND Score > 80", search_city);
    println!("インデックス: (City, Age)");
    println!("=======================================================");

    // 1. ICPなしの実行
    let _ = classic_storage_lookup(&composite_index, &users_table, search_city);

    println!("\n-------------------------------------------------------");
    println!("  🔥 **注目ポイント:** ICPによりテーブルアクセス回数が減少！");
    println!("-------------------------------------------------------");
    
    // 2. ICPありの実行
    let _ = icp_storage_lookup(&composite_index, &users_table, search_city);
}