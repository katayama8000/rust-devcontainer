use std::collections::BTreeMap;
use std::time::Instant; 

// 仮のRDBテーブルの行構造
#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
    age: u8,
    city: String, 
}

// ユーザーテーブル全体 (メモリ上の配列で代用)
type UsersTable = Vec<User>;

// B-treeインデックスの定義: BTreeMapはRustにおけるソートされた木構造（B-treeに類似）
type CityIndex = BTreeMap<String, Vec<u32>>;

// ダミーデータとインデックスのセットアップ関数
fn setup_data() -> (UsersTable, CityIndex) {
    let mut table = UsersTable::new();
    let mut index = CityIndex::new();

    // データ量を増やしてコスト差をより明確にします（例：1000件）
    let total_data = 1000;
    let cities = vec!["東京", "大阪", "福岡", "札幌"];
    
    for i in 0..total_data {
        let city = cities[i % cities.len()];
        
        let user = User {
            id: i as u32 + 1,
            name: format!("User{}", i + 1),
            age: (20 + i % 5) as u8,
            city: city.to_string(),
        };
        
        let row_index = i as u32; 
        table.push(user);

        // BTreeMapによるインデックス構築
        index.entry(city.to_string())
             .or_insert_with(Vec::new)
             .push(row_index);
    }
    (table, index)
}

fn search_without_index(table: &UsersTable, city: &str) -> Vec<User> {
    println!("\n=======================================================");
    println!(" A. 処理: インデックスなし (フルテーブルスキャン)");
    println!("=======================================================");
    let start_time = Instant::now(); 
    
    let mut results = Vec::new();
    
    for user in table.iter() {
        if user.city == city {
            results.push(user.clone());
        }
    }

    let duration = start_time.elapsed(); 
    
    // ----------------- コストの表示 -----------------
    println!("--- 🔍 検索コスト (N={}) ---", table.len());
    println!("  -> 比較/チェックした行数: **{} 行** (テーブル全件スキャン)", table.len()); 
    println!("  -> 処理時間 (目安): **{:?}**", duration);
    
    // ----------------- 結果の表示 -----------------
    println!("--- ✅ 検索結果 ---");
    println!("  -> 見つかった件数: **{} 件**", results.len());
    
    results
}

fn search_with_btree_index(table: &UsersTable, index: &CityIndex, city: &str) -> Vec<User> {
    println!("\n=======================================================");
    println!(" B. 処理: **B-treeインデックス (BTreeMap)** を使用");
    println!("=======================================================");
    let start_time = Instant::now();
    
    let mut results = Vec::new();

    // 1. BTreeMap (B-tree) でキーを検索: O(log N)の高速検索
    match index.get(city) {
        Some(row_indices) => {
            
            // 2. 取得した行インデックスを使ってテーブルのデータへ直行
            for &i in row_indices {
                if let Some(user) = table.get(i as usize) {
                    results.push(user.clone());
                }
            }
            
            let duration = start_time.elapsed();

            // ----------------- コストの表示 -----------------
            println!("--- 🔍 検索コスト (N={}) ---", table.len());
            println!("  -> ステップ1: **B-tree** 検索: O(log N) - **高速**");
            println!("  -> ステップ2: テーブルへのアクセス回数: **{} 回** (結果件数分のみ)", row_indices.len());
            println!("  -> 処理時間 (目安): **{:?}**", duration);

        }
        None => {
            let duration = start_time.elapsed();
            println!("--- 🔍 検索コスト ---");
            println!("  -> B-tree 検索の結果、データは見つかりませんでした。");
            println!("  -> 処理時間 (目安): **{:?}**", duration);
        }
    }

    // ----------------- 結果の表示 -----------------
    println!("--- ✅ 検索結果 ---");
    println!("  -> 見つかった件数: **{} 件**", results.len());
    
    results
}

// =======================================================
// run 関数
// =======================================================
pub fn run() {
    println!("=======================================================");
    println!("   RDBインデックスによる検索コスト比較デモ 🚀");
    println!("=======================================================");
    
    let (users_table, city_index) = setup_data();
    let search_city = "東京";

    println!("総データ件数: **{} 件**", users_table.len());
    println!("検索対象都市: **【{}】**", search_city);

    // ---------------------------------------------------
    // ⭐ B-treeインデックス（CityIndex）の中身を出力
    // ---------------------------------------------------
    println!("\n=======================================================");
    println!("   **B-treeインデックス** の構造（中身の可視化）");
    println!("   (B-treeはキーでソートされた構造になっています)");
    println!("=======================================================");
    
    for (city, indices) in city_index.iter() {
        let sample_indices = if indices.len() > 5 {
            format!("{:?}...", &indices[..5]) // 5件まで表示し、省略
        } else {
            format!("{:?}", indices)
        };
        
        println!("【B-treeのキー: {}】 -> 件数: {}件, テーブルの行番号 (インデックス): {}", 
                 city, indices.len(), sample_indices);
    }
    println!("（キーがソートされているため、**O(log N)の高速検索**が可能です）");
    println!("=======================================================");


    // 実行比較
    let _ = search_without_index(&users_table, search_city);
    let _ = search_with_btree_index(&users_table, &city_index, search_city);
    
    println!("\n=======================================================");
    println!("   最終結果確認");
    println!("=======================================================");
    println!("（大規模なデータで比較すると、処理時間の差が顕著になります）");
    println!("=======================================================");
}