use std::collections::BTreeMap;

// =======================================================
// 1. データ構造の定義
// =======================================================

#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
    // ↓カーディナリティが低いフィールド (性別)
    gender: String, 
    // ↓カーディナリティが高いフィールド (Eメール)
    email: String, 
}

type UsersTable = Vec<User>;

// === インデックスの定義 ===
// 性別に対するインデックス (低カーディナリティ)
type GenderIndex = BTreeMap<String, Vec<u32>>;

// Eメールに対するインデックス (高カーディナリティ)
type EmailIndex = BTreeMap<String, Vec<u32>>;

// =======================================================
// 2. データとインデックスのセットアップ
// =======================================================

fn setup_data() -> (UsersTable, GenderIndex, EmailIndex) {
    let mut table = UsersTable::new();
    let mut gender_index = GenderIndex::new();
    let mut email_index = EmailIndex::new();

    let total_data = 1000;
    let genders = vec!["男性", "女性", "その他"]; // 非常に少ないバリエーション

    for i in 0..total_data {
        let gender = genders[i % genders.len()].to_string();
        let email = format!("user{}@example.com", i); // ほぼ全てユニーク

        let user = User {
            id: i as u32 + 1,
            name: format!("User{}", i + 1),
            gender: gender.clone(),
            email: email.clone(),
        };

        let row_index = i as u32;
        table.push(user);

        // 性別インデックスの構築
        gender_index.entry(gender).or_insert_with(Vec::new).push(row_index);
        
        // Eメールインデックスの構築
        email_index.entry(email).or_insert_with(Vec::new).push(row_index);
    }

    (table, gender_index, email_index)
}

// =======================================================
// 3. インデックス構造の可視化と説明
// =======================================================

fn print_index_structure(gender_index: &GenderIndex, email_index: &EmailIndex) {
    println!("\n=======================================================");
    println!("   インデックスのカーディナリティ比較");
    println!("=======================================================");

    // --- 低カーディナリティの例 (性別) ---
    println!("\n--- A. 低カーディナリティのインデックス (Gender) ---");
    println!("-> キーの種類が非常に少ない (重複が多い)");
    println!("-------------------------------------------------------");
    for (key, indices) in gender_index.iter() {
        println!("【キー: {}】 -> {} 件の行がヒット", key, indices.len());
    }
    println!("\n[解説] 性別のようなカーディナリティが低い列にインデックスを張ると…");
    println!("  - インデックスのキーが少なく、一つのキーに多数の行がぶら下がる。");
    println!("  - 例えば「男性」で検索しても、全データの約半分がヒットするため、絞り込み効果が薄い。");
    println!("  - このような場合、インデックスを使わずにフルスキャンした方が速いことさえある。");

    // --- 高カーディナリティの例 (Eメール) ---
    println!("\n--- B. 高カーディナリティのインデックス (Email) ---");
    println!("-> キーの種類が非常に多い (ほぼユニーク)");
    println!("-------------------------------------------------------");
    let mut count = 0;
    for (key, indices) in email_index.iter() {
        if count < 5 { // 最初の5件だけ表示
            println!("【キー: {}】 -> {} 件の行がヒット", key, indices.len());
        }
        count += 1;
    }
    println!("... (多数のユニークなキーが続く) ...");
    println!("インデックスに含まれるキーの総数: {} 件", email_index.len());

    println!("\n[解説] Eメールのようなカーディナリティが高い列にインデックスを張ると…");
    println!("  - インデックスのキーがほぼユニークで、一つのキーに少数の行しか対応しない (理想は1行)。");
    println!("  - 特定のEメールで検索すると、目的の行に直接アクセスできるため、非常に高速。");
    println!("  - これがインデックスが最も効果を発揮する典型的なケース。");
}

// =======================================================
// 4. 実行
// =======================================================

pub fn run() {
    println!("=======================================================");
    println!("   RDBのカーディナリティ理解デモ 🚀");
    println!("=======================================================");
    
    let (_users_table, gender_index, email_index) = setup_data();

    print_index_structure(&gender_index, &email_index);

    println!("\n=======================================================");
    println!("   結論");
    println!("=======================================================");
    println!("インデックスは「カーディナリティが高い」列 (ID, Eメールなど) に作成すると最も効果的です。");
    println!("逆に「カーディナリティが低い」列 (性別, フラグなど) では効果が薄いか、逆効果になることもあります。");
    println!("=======================================================");
}
