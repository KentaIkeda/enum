// enumキーワードを使用して、列挙型を作成
enum IpAddressKind {
    V4,
    V6
}

struct IpAddressStructure {
    kind: IpAddressKind,
    address: String
}

fn main() {
    // enumを使用する際には::演算子を使用して、アクセスする
    // IpAddressKind型を受け取る関数にパラメータとして渡せる
    let four = IpAddressKind::V4;
    let six = IpAddressKind::V6;
    // こんな感じで
    route(four);
    route(six);

    let home = IpAddressStructure {
        kind: IpAddressKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddressStructure {
        kind: IpAddressKind::V6,
        address: String::from("::1"),
    };
}

fn route(ip: IpAddressKind) -> IpAddressKind {
    ip
}