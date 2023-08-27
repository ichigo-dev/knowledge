# 『論理回路』ノート

（最終更新： 2023-03-07）


## 目次

1. [論理回路](#論理回路)
1. [フリップフロップ回路](#フリップフロップ回路)
1. [加算回路](#加算回路)
	1. [半加算回路](#半加算回路)
	1. [全加算回路](#全加算回路)
	1. [多数決回路](#多数決回路)


## 論理回路

**論理回路**は、[論理演算](../../../../basics/discrete_mathematics/_/chapters/logical_operation.md#論理演算)を行う電気回路のことで、論理回路を組み合わせることで複雑な回路を構成することができる。論理回路は**MIL記号**により表記される。

代表的な論理回路には、[OR演算](../../../../basics/discrete_mathematics/_/chapters/logical_operation.md#or演算)を行う**OR回路**、[AND演算](../../../../basics/discrete_mathematics/_/chapters/logical_operation.md#and演算)を行う**AND回路**、[NOT演算](../../../../basics/discrete_mathematics/_/chapters/logical_operation.md#not演算)を行う**NOT回路**、[NOR演算](../../../../basics/discrete_mathematics/_/chapters/logical_operation.md#nor演算)を行う**NOR回路**、[NAND演算](../../../../basics/discrete_mathematics/_/chapters/logical_operation.md#nand演算)を行う**NAND回路**、[XOR演算](../../../../basics/discrete_mathematics/_/chapters/logical_operation.md#xor演算)を行う**XOR回路**がある。中でもNAND回路は他の回路に比べて回路構成が簡単で作りやすいという特徴があり、他の論理回路もすべてNAND回路のみを組み合わせて作ることができる。


## フリップフロップ回路

**フリップフロップ回路**は、1ビットの情報を記憶することができる論理回路。SRAMなどでよく利用されており、NAND回路を2つ組み合わせた**SR型**(Set Reset)フリップフロップや、NAND回路を4つ組み合わせた**JK型**フリップフロップがある。


## 加算回路

**加算回路**は、[コンピュータ](../../../_/chapters/computer.md#コンピュータ)の計算の基本となる加算を行うための回路。[コンピュータ](../../../_/chapters/computer.md#コンピュータ)では[補数](../../../../basics/discrete_mathematics/_/chapters/numeric_representation.md#補数)を用いることによって、減算も加算により計算することができる。

### 半加算回路

**半加算回路**(HA: Half Adder)は、1桁の[2進数](../../../../basics/discrete_mathematics/_/chapters/radix.md#2進数)の加算を行う回路で、桁上がりの数を考慮する。2つの入力を受け付け、出力は演算結果と桁上がり結果となる。

### 全加算回路

**全加算回路**(FA: Full Adder)は、入力に下位桁からの桁上がりを含め、1桁の[2進数](../../../../basics/discrete_mathematics/_/chapters/radix.md#2進数)の加算を行う回路。下位桁の桁上がりを含めた3つの入力を受け付け、出力は演算結果と桁上がり結果となる。

### 多数決回路

**多数決回路**は、[2進数](../../../../basics/discrete_mathematics/_/chapters/radix.md#2進数)1桁の値が複数入力され、その中でも多い方の入力と同じ値を出力する回路。3入力多数決回路であれば、2つ以上同じ値であればその値を出力する。


## 参考文献

- [角谷一成.令和05年 基本情報技術者合格教本.株式会社技術評論社, 2022, 575](https://gihyo.jp/book/2022/978-4-297-13164-7)
- [瀬戸美月.徹底攻略 応用情報技術者教科書 令和4年度.株式会社インプレス, 2021, 814](https://book.impress.co.jp/books/1121101057)
