use std::net::UdpSocket;
use std::collections::HashMap;

// DNSヘッダー構造（簡易版）
#[derive(Debug, Clone)]
struct DnsHeader {
    id: u16,              // トランザクションID
    flags: u16,           // フラグ (QR, Opcode, AA, TC, RD, RA, Z, RCODE)
    qd_count: u16,        // 質問セクションのエントリ数
    an_count: u16,        // 応答セクションのエントリ数
    ns_count: u16,        // 権威セクションのエントリ数
    ar_count: u16,        // 追加情報セクションのエントリ数
}

impl DnsHeader {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.id.to_be_bytes());
        bytes.extend_from_slice(&self.flags.to_be_bytes());
        bytes.extend_from_slice(&self.qd_count.to_be_bytes());
        bytes.extend_from_slice(&self.an_count.to_be_bytes());
        bytes.extend_from_slice(&self.ns_count.to_be_bytes());
        bytes.extend_from_slice(&self.ar_count.to_be_bytes());
        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < 12 {
            return Err("ヘッダーが短すぎます".to_string());
        }

        Ok(DnsHeader {
            id: u16::from_be_bytes([bytes[0], bytes[1]]),
            flags: u16::from_be_bytes([bytes[2], bytes[3]]),
            qd_count: u16::from_be_bytes([bytes[4], bytes[5]]),
            an_count: u16::from_be_bytes([bytes[6], bytes[7]]),
            ns_count: u16::from_be_bytes([bytes[8], bytes[9]]),
            ar_count: u16::from_be_bytes([bytes[10], bytes[11]]),
        })
    }
}

// DNSレコードタイプ
#[derive(Debug, Clone, Copy, PartialEq)]
enum RecordType {
    A = 1,      // IPv4アドレス
    NS = 2,     // ネームサーバー
    CNAME = 5,  // 正規名
    AAAA = 28,  // IPv6アドレス
}

// DNSクエリの解析結果
#[derive(Debug)]
struct DnsQuery {
    name: String,
    qtype: u16,
    qclass: u16,
}

// 簡易DNSサーバー
pub struct DnsServer {
    socket: UdpSocket,
    records: HashMap<String, Vec<u8>>, // ドメイン名 -> IPアドレス (4バイト)
}

impl DnsServer {
    pub fn new(bind_addr: &str) -> Result<Self, std::io::Error> {
        let socket = UdpSocket::bind(bind_addr)?;
        let mut records = HashMap::new();
        
        // サンプルレコードを登録
        records.insert("example.com".to_string(), vec![93, 184, 216, 34]);     // 93.184.216.34
        records.insert("localhost".to_string(), vec![127, 0, 0, 1]);           // 127.0.0.1
        records.insert("test.local".to_string(), vec![192, 168, 1, 100]);      // 192.168.1.100
        records.insert("myserver.local".to_string(), vec![10, 0, 0, 50]);      // 10.0.0.50
        
        println!("🚀 DNS Server started on {}", bind_addr);
        println!("📋 登録されているレコード:");
        for (domain, ip) in &records {
            println!("  {} -> {}.{}.{}.{}", domain, ip[0], ip[1], ip[2], ip[3]);
        }
        
        Ok(DnsServer { socket, records })
    }

    pub fn run(&self) -> Result<(), std::io::Error> {
        let mut buf = [0u8; 512];
        
        println!("\n⏳ クエリを待機中...\n");
        
        loop {
            let (size, src) = self.socket.recv_from(&mut buf)?;
            println!("\n{}", "=".repeat(60));
            println!("📨 [1/8] クエリ受信");
            println!("  サイズ: {} バイト", size);
            println!("  送信元: {}", src);
            println!("  生データ (最初の16バイト): {:02X?}", &buf[..16.min(size)]);
            println!("{}", "=".repeat(60));
            
            // クエリを処理して応答を生成
            match self.process_query(&buf[..size]) {
                Ok(response) => {
                    println!("\n📤 [7/8] 応答送信準備");
                    println!("  応答サイズ: {} バイト", response.len());
                    println!("  送信先: {}", src);
                    println!("  応答データ (最初の16バイト): {:02X?}", &response[..16.min(response.len())]);
                    
                    self.socket.send_to(&response, src)?;
                    
                    println!("\n✅ [8/8] 応答送信完了");
                    println!("{}\n", "=".repeat(60));
                }
                Err(e) => {
                    println!("\n❌ エラー発生: {}", e);
                    println!("{}\n", "=".repeat(60));
                }
            }
        }
    }

    fn process_query(&self, query: &[u8]) -> Result<Vec<u8>, String> {
        // ヘッダーを解析
        println!("\n🔍 [2/8] DNS ヘッダー解析");
        let header = DnsHeader::from_bytes(query)?;
        println!("  トランザクションID: 0x{:04X} ({})", header.id, header.id);
        println!("  フラグ: 0x{:04X}", header.flags);
        println!("  質問数 (QDCOUNT): {}", header.qd_count);
        println!("  応答数 (ANCOUNT): {}", header.an_count);
        println!("  権威数 (NSCOUNT): {}", header.ns_count);
        println!("  追加数 (ARCOUNT): {}", header.ar_count);
        
        // 質問セクションを解析（簡易版: 最初の質問のみ処理）
        println!("\n🔎 [3/8] 質問セクション解析");
        let question = self.parse_question(&query[12..])?;
        println!("  ドメイン名: {}", question.name);
        println!("  クエリタイプ: {} ({})", question.qtype, 
                 if question.qtype == 1 { "A (IPv4)" } 
                 else if question.qtype == 28 { "AAAA (IPv6)" }
                 else { "その他" });
        println!("  クエリクラス: {} ({})", question.qclass,
                 if question.qclass == 1 { "IN (インターネット)" } else { "その他" });
        
        // レコードを検索
        println!("\n🗂️  [4/8] レコード検索");
        println!("  検索キー: '{}'", question.name);
        println!("  登録レコード数: {}", self.records.len());
        
        // 全レコードを表示
        println!("  📚 登録済みレコード一覧:");
        for (domain, ip) in &self.records {
            println!("    - {} -> {}.{}.{}.{}", domain, ip[0], ip[1], ip[2], ip[3]);
        }
        
        let ip_addr = self.records.get(&question.name);
        
        if let Some(ip) = ip_addr {
            println!("  ✅ 検索結果: 見つかりました");
            println!("  マッチしたレコード: {} -> {}.{}.{}.{}", question.name, ip[0], ip[1], ip[2], ip[3]);
        } else {
            println!("  ❌ 検索結果: 見つかりません");
            println!("  '{}' に対応するレコードは登録されていません", question.name);
        }
        
        // 応答ヘッダーを作成
        println!("\n📋 [5/8] 応答ヘッダー作成");
        let mut response_header = header.clone();
        response_header.flags = 0x8180; // 標準クエリ応答、再帰利用可能
        response_header.an_count = if ip_addr.is_some() { 1 } else { 0 };
        
        if ip_addr.is_none() {
            // NXDOMAIN (存在しないドメイン)
            response_header.flags |= 0x0003;
            println!("  ステータス: NXDOMAIN (ドメイン不明)");
            println!("  応答フラグ: 0x{:04X}", response_header.flags);
            println!("  応答レコード数: 0");
        } else {
            println!("  ステータス: NOERROR (成功)");
            println!("  応答フラグ: 0x{:04X}", response_header.flags);
            println!("  応答レコード数: 1");
        }
        
        // 応答を構築
        println!("\n🔧 [6/8] 応答パケット構築");
        let mut response = response_header.to_bytes();
        println!("  ヘッダー: {} バイト追加", response.len());
        
        // 質問セクションをそのままコピー
        let question_end = 12 + self.get_question_length(&query[12..])?;
        let question_len = question_end - 12;
        response.extend_from_slice(&query[12..question_end]);
        println!("  質問セクション: {} バイト追加 (合計: {}バイト)", question_len, response.len());
        
        // 応答セクションを追加
        if let Some(ip) = ip_addr {
            let answer = self.create_answer(&question.name, ip);
            println!("  応答セクション: {} バイト追加", answer.len());
            response.extend_from_slice(&answer);
            println!("  最終パケットサイズ: {} バイト", response.len());
        } else {
            println!("  応答セクション: なし (NXDOMAIN)");
            println!("  最終パケットサイズ: {} バイト", response.len());
        }
        
        Ok(response)
    }

    fn parse_question(&self, data: &[u8]) -> Result<DnsQuery, String> {
        let (name, offset) = self.parse_domain_name(data)?;
        
        if data.len() < offset + 4 {
            return Err("質問セクションが不完全です".to_string());
        }
        
        let qtype = u16::from_be_bytes([data[offset], data[offset + 1]]);
        let qclass = u16::from_be_bytes([data[offset + 2], data[offset + 3]]);
        
        Ok(DnsQuery { name, qtype, qclass })
    }

    fn parse_domain_name(&self, data: &[u8]) -> Result<(String, usize), String> {
        let mut labels = Vec::new();
        let mut pos = 0;
        
        println!("    📝 ドメイン名パース開始");
        
        loop {
            if pos >= data.len() {
                return Err("ドメイン名が不完全です".to_string());
            }
            
            let len = data[pos] as usize;
            if len == 0 {
                println!("    📝 終端バイト (0x00) 検出");
                pos += 1;
                break;
            }
            
            pos += 1;
            if pos + len > data.len() {
                return Err("ドメイン名が不完全です".to_string());
            }
            
            let label = String::from_utf8_lossy(&data[pos..pos + len]).to_string();
            println!("    📝 ラベル検出: '{}' (長さ: {})", label, len);
            labels.push(label);
            pos += len;
        }
        
        let domain = labels.join(".");
        println!("    📝 完成したドメイン名: '{}' (合計: {}バイト)", domain, pos);
        
        Ok((domain, pos))
    }

    fn get_question_length(&self, data: &[u8]) -> Result<usize, String> {
        let (_, offset) = self.parse_domain_name(data)?;
        Ok(offset + 4) // ドメイン名 + QTYPE(2) + QCLASS(2)
    }

    fn create_answer(&self, domain: &str, ip: &[u8]) -> Vec<u8> {
        let mut answer = Vec::new();
        
        // ドメイン名をエンコード
        for label in domain.split('.') {
            answer.push(label.len() as u8);
            answer.extend_from_slice(label.as_bytes());
        }
        answer.push(0); // 終端
        
        // TYPE (A record = 1)
        answer.extend_from_slice(&(RecordType::A as u16).to_be_bytes());
        
        // CLASS (IN = 1)
        answer.extend_from_slice(&1u16.to_be_bytes());
        
        // TTL (Time To Live: 300秒)
        answer.extend_from_slice(&300u32.to_be_bytes());
        
        // RDLENGTH (IPv4は4バイト)
        answer.extend_from_slice(&4u16.to_be_bytes());
        
        // RDATA (IPアドレス)
        answer.extend_from_slice(ip);
        
        answer
    }
}

// =======================================================
// run 関数
// =======================================================
pub fn run() {
    println!("=======================================================");
    println!("   簡易DNSサーバー デモ 🌐");
    println!("=======================================================\n");
    
    // DNSサーバーを起動 (UDP ポート 9053で待機)
    // 注: 通常のDNSはポート53ですが、権限が必要なため9053を使用
    match DnsServer::new("127.0.0.1:9053") {
        Ok(server) => {
            println!("💡 テスト方法:");
            println!("   別のターミナルで以下を実行:");
            println!("   dig @127.0.0.1 -p 9053 example.com");
            println!("   dig @127.0.0.1 -p 9053 localhost");
            println!("   dig @127.0.0.1 -p 9053 test.local");
            println!("\n   または nslookup を使用:");
            println!("   nslookup -port=9053 example.com 127.0.0.1");
            println!("=======================================================\n");
            
            if let Err(e) = server.run() {
                eprintln!("❌ サーバーエラー: {}", e);
            }
        }
        Err(e) => {
            eprintln!("❌ サーバーの起動に失敗: {}", e);
        }
    }
}
