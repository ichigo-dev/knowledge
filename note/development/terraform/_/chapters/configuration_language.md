# 『設定言語』ノート

（最終更新： 2023-10-06）


## 目次

1. [設定言語](#設定言語)
	1. [Argument](#argument)
	1. [Block](#block)
1. [モジュール](#モジュール)
	1. [ルートモジュール](#ルートモジュール)


## 設定言語

[Terraform](./terraform.md#terraform)の設定言語の目的は、インフラストラクチャオブジェクトを表すリソースを宣言することであり、リソース定義をより柔軟かつ便利に行えるように設計されている。

設定言語の基本的な構文は以下のように構成される。

```terraform
# 例
resource "aws_vpc" "main" {
    cidr_block = var.base_cidr_block
}

# 構成
<BLOCK TYPE> "<BLOCK LABEL>" "<BLOCK LABEL>" {
    # ブロックのボディ部
    <IDENTIFIER> = <EXPRESSION>
}
```

[Terraform](./terraform.md#terraform)ではブロックが宣言される順番は重要ではなく、リソース間の関係のみを参照して操作の順序が決定される。

### Argument

**Argument**は、[Terraform](./terraform.md#terraform)において、特定の名前に対して値を割り当てるために用いられる構文。

```terraform
resource_id = "abc123"
```

等号の前の識別子は名前、後の式は値として扱われる。

### Block

**Block**は、[Terraform](./terraform.md#terraform)において、他のコンテンツを包み込むコンテナを宣言する構文。Blockには、そのを表すタイプと、0個以上のラベルが定義される。また、Blockはネストすることもできる。

```terraform
resource "aws_instance" "example" {
    ami = "abc123"

    network_interface {
        # ...
    }
}
```

[Terraform](./terraform.md#terraform)にはいくつかの決められたBlockタイプがあり、これらを組み合わせて様々な機能を実装することができる。


## モジュール

**モジュール**は、[Terraform](./terraform.md#terraform)において、ひとつの[ディレクトリ](../../../../computer/software/_/chapters/file_system.md#ディレクトリ)に含まれる[Terraform](./terraform.md#terraform)構成[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)の集まり。[Terraform](./terraform.md#terraform)の構成[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)は、[設定言語](#設定言語)を用いて記述し、拡張子を `.tf` として作成する。

[Terraform](./terraform.md#terraform)では、モジュール内の構成[ファイル](../../../../computer/software/_/chapters/file_system.md#ファイル)の読み込み順を意識する必要はなく、モジュール全体が単一のドキュメントとして扱われるようになっている。また、モジュール内に[ディレクトリ](../../../../computer/software/_/chapters/file_system.md#ディレクトリ)を含めた場合、それは無視されて完全に別のモジュールとして扱われる。これを含めたい場合は明示的にモジュール呼び出しを使用する必要がある。

### ルートモジュール

**ルートモジュール**は、[Terraform](./terraform.md#terraform)において、処理の実行時に基準となる[モジュール](#モジュール)。[Terraform](./terraform.md#terraform)の処理は、常にひとつのモジュールを基準にして実行され、[CLI](./cli.md#cli)ではデフォルトで[カレントディレクトリ](../../../../computer/software/_/chapters/file_system.md#カレントディレクトリ)をルートモジュールとしている。


## resource

**resource**は、[Terraform](./terraform.md#terraform)において、仮想ネットワークやコンピューティングインスタンスなどの1つ以上のインフラストラクチャオブジェクトを記述するための[Block](#block)。

```terraform
resource "aws_instance" "web" {
    ami           = "abc123"
    instance_type = "t2.micro"
}
```
