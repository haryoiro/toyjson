#toyjson

JSONパーサの実装

## PrimitiveType

- string
  - Unicodeに対応
  -
- number
- boolean
- null

## 複合型

- array
- object

ws = (
    %x20 / ; Space
    %x09 / ; Horizontal Tab
    %x0A / ; Line feed or New Line
    %x0D / ; Carriage return
)

## 字句解析器を実装

文字列をトークン化し、構文解析器にかけられる形式に変換する装置。英語でLexer。
基本的には文字列をイテレータとして取り込み、一文字ずつ処理していく。文字列やエスケープシーケンスなど、一文字では完結しないトークンにあたった場合はpeekで次の文字を読み込み、そのトークンの終わり（"}"や"]"など）にたどりついたらトークンに変換する。

また、エラーの場所を知りたいのでトークン毎に`Location`構造体を付与していく。

複数文字のトークンの構造　RFCより抜粋

```number.rs

number = [ minus ] int [ fcac ]

decimal-point = %x2E    ; .
digit1-9 = %x31-39      ; 1-9
e = %x65 / %x45         ; e E
exp = e [ minus / plus ] 1*DIGIT

frac = decimal-point 1*DIGIT
int = zero / ( digit1-9 *DIGIT )

minus = %x2D
plus = %x2B
zero = %x30

```

JSONで許可されている数値表現は以下の
- 整数
- 浮動小数点
- 指数表記

## 文字列

引用符で囲まれた範囲はエスケープする必要がある文字を除き、すべてのUnicode文字を使用可能である。

```string.rs

char = unescaped /
    escape (
        %x22 /      ; " quote
        %x5C /      ; \ reverse solidus
        %x2F /      ; / solidus
        %x62 /      ; b backspace
        %x66 /      ; f form feed
        %x6E /      ; n line feed
        %x72 /      ; r carriage return
        %x74 /      ; t tab
        %x75 /      ; uXXXX
        %x
        %x
    )

escape = %x5C           ; \
quotation-mark = %x22   ; "
unescaped = %x20-21 / %x23-5B / %x5D-10FFFF

```

## Token

```token.rs

struct Token {
    location: Location,
    token_type: TokenType,
}

enum Value {
    String(String),
    Number(usize),
    Boolean(Boolean),
    Object(Object),
    Array(Array)
}

enum Boolean{
    True,
    False,
}

struct Object((String, Value))
struct Array(Vec<Value>)

```
