# 問題集

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
<details>
<summary>Table of Contents</summary>

- [準備問題](#%E6%BA%96%E5%82%99%E5%95%8F%E9%A1%8C)
- [全探索](#%E5%85%A8%E6%8E%A2%E7%B4%A2)
- [再帰と分割統治法](#%E5%86%8D%E5%B8%B0%E3%81%A8%E5%88%86%E5%89%B2%E7%B5%B1%E6%B2%BB%E6%B3%95)
- [動的計画法](#%E5%8B%95%E7%9A%84%E8%A8%88%E7%94%BB%E6%B3%95)
- [二分探索法](#%E4%BA%8C%E5%88%86%E6%8E%A2%E7%B4%A2%E6%B3%95)
- [貪欲法](#%E8%B2%AA%E6%AC%B2%E6%B3%95)
- [配列、連結リスト、ハッシュテーブル](#%E9%85%8D%E5%88%97%E9%80%A3%E7%B5%90%E3%83%AA%E3%82%B9%E3%83%88%E3%83%8F%E3%83%83%E3%82%B7%E3%83%A5%E3%83%86%E3%83%BC%E3%83%96%E3%83%AB)
- [スタック、キュー](#%E3%82%B9%E3%82%BF%E3%83%83%E3%82%AF%E3%82%AD%E3%83%A5%E3%83%BC)
- [グラフと木](#%E3%82%B0%E3%83%A9%E3%83%95%E3%81%A8%E6%9C%A8)
- [Union-Find](#union-find)
- [グラフ探索](#%E3%82%B0%E3%83%A9%E3%83%95%E6%8E%A2%E7%B4%A2)
- [最短路](#%E6%9C%80%E7%9F%AD%E8%B7%AF)
- [最小全域木](#%E6%9C%80%E5%B0%8F%E5%85%A8%E5%9F%9F%E6%9C%A8)
- [ネットワークフロー](#%E3%83%8D%E3%83%83%E3%83%88%E3%83%AF%E3%83%BC%E3%82%AF%E3%83%95%E3%83%AD%E3%83%BC)
- [参考資料](#%E5%8F%82%E8%80%83%E8%B3%87%E6%96%99)

</details>
<!-- END doctoc generated TOC please keep comment here to allow auto update -->

## 準備問題

- 倍数判定
  - [x] [ABC 086 A - Product](https://atcoder.jp/contests/abc086/tasks/abc086_a)
  - [x] [ABC 064 A - RGB Cards](https://atcoder.jp/contests/abc064/tasks/abc064_a)
  - [x] [ABC 088 A - Infinite Coins](https://atcoder.jp/contests/abc088/tasks/abc088_a)
  - [x] [ABC 157 A - Duplex Printing](https://atcoder.jp/contests/abc157/tasks/abc157_a)
- 文字列操作
  - [x] [ABC 081 A - Placing Marbles](https://atcoder.jp/contests/abc081/tasks/abc081_a)
  - [x] [ABC 095 A - Something on It](https://atcoder.jp/contests/abc095/tasks/abc095_a)
  - [x] [ABC 085 A - Already 2018](https://atcoder.jp/contests/abc085/tasks/abc085_a)
  - [x] [ABC 069 B - i18n](https://atcoder.jp/contests/abc069/tasks/abc069_b)
  - [x] [ABC 082 B - Two Anagrams](https://atcoder.jp/contests/abc082/tasks/abc082_b)
- N進数
  - [x] [ABC 083 B - Some Sums](https://atcoder.jp/contests/abc083/tasks/abc083_b)
  - [x] [ABC 090 B - Palindromic Numbers](https://atcoder.jp/contests/abc090/tasks/abc090_b)
  - [x] [AGC 025 A - Digits Sum](https://atcoder.jp/contests/agc025/tasks/agc025_a)
  - [x] [ABC 156 B - Digits](https://atcoder.jp/contests/abc156/tasks/abc156_b)
- ソート
  - [x] [ABC 088 B - Card Game for Two](https://atcoder.jp/contests/abc088/tasks/abc088_b)
  - [x] [ABC 067 B - Snake Toy](https://atcoder.jp/contests/abc067/tasks/abc067_b)
  - [x] [ABC 042 B - Iroha Loves Strings](https://atcoder.jp/contests/abc042/tasks/abc042_b)
  - [ ] [AGC 027 A - Candy Distribution Again](https://atcoder.jp/contests/agc027/tasks/agc027_a)
  - [ ] [AGC 012 A - AtCoder Group Contest](https://atcoder.jp/contests/agc012/tasks/agc012_a)
- 連想配列
  - [ ] [ABC 085 B - Kagami Mochi](https://atcoder.jp/contests/abc085/tasks/abc085_b)
  - [x] [ABC 071 B - Not Found](https://atcoder.jp/contests/abc071/tasks/abc071_b)
  - [x] [ABC 061 B - Counting Roads](https://atcoder.jp/contests/abc061/tasks/abc061_b)
  - [ ] [ABC 047 B - Snuke's Coloring 2 (ABC Edit)](https://atcoder.jp/contests/abc047/tasks/abc047_b)
  - [ ] [ABC 091 B Two Colors Card Game](https://atcoder.jp/contests/abc091/tasks/abc091_b)
  - [ ] [ABC 081 C - Not so Diverse](https://atcoder.jp/contests/abc081/tasks/arc086_a)
- パリティ
  - [ ] [ABC 086 C - Traveling](https://atcoder.jp/contests/abc086/tasks/arc089_a)
  - [ ] [ABC 093 C - Same Integers](https://atcoder.jp/contests/abc093/tasks/arc094_a)
  - [ ] [AGC 010 A - Addition](https://atcoder.jp/contests/agc010/tasks/agc010_a)
  - [ ] [AGC 020 A - Move and Win](https://atcoder.jp/contests/agc020/tasks/agc020_a)
  - [ ] [ABC 073 C - Write and Erase](https://atcoder.jp/contests/abc073/tasks/abc073_c)
- グリッド
  - [ ] [ABC 075 B](https://atcoder.jp/contests/abc075/tasks/abc075_b)
  - [ ] [ABC 096 C](https://atcoder.jp/contests/abc096/tasks/abc096_c)
- 区間重なり
  - [ ] [ABC 070 B](https://atcoder.jp/contests/abc070/tasks/abc070_b)
- 余り
  - [ ] [ABC 055 B](https://atcoder.jp/contests/abc055/tasks/abc055_b)
- 数学的問題
  - [ ] [ABC 046 B](https://atcoder.jp/contests/abc046/tasks/abc046_b)
  - [ ] [ABC 048 B](https://atcoder.jp/contests/abc048/tasks/abc048_b)
- 状態ループ
  - [ ] [ABC 060 B](https://atcoder.jp/contests/abc060/tasks/abc060_b)
  - [ ] [ABC 065 B](https://atcoder.jp/contests/abc065/tasks/abc065_b)

参考記事

[累積和を何も考えずに書けるようにする！](https://qiita.com/drken/items/56a6b68edef8fc605821)
[累積和を使うAtCoderの問題まとめと実装（C++）](https://qiita.com/takuya000885/items/8881ba05d8e6475a1759)

- 累積和
  - [ ] [AOJ 0516 - 最大の和 (JOI 2006 本選 A)](http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=0516)
  - [ ] [全国統一プログラミング王決定戦本線 A - Abundant Resources](https://atcoder.jp/contests/nikkei2019-final/tasks/nikkei2019_final_a)
  - [ ] [ABC 037 C- 総和](https://atcoder.jp/contests/abc037/tasks/abc037_c)
  - [ ] [ABC 134 C - Exception Handling](https://atcoder.jp/contests/abc134/tasks/abc134_c)
  - [ ] [ABC 098 C - Attention](https://atcoder.jp/contests/abc098/tasks/arc098_a)
  - [ ] [Tenka1 Programmer Contest 2019 C - Stones](https://atcoder.jp/contests/tenka1-2019-beginner/tasks/tenka1_2019_c)
  - [ ] [ABC 084 D - 2017-like Number](https://atcoder.jp/contests/abc084/tasks/abc084_d)
  - [ ] [ABC 122 C - GeT AC](https://atcoder.jp/contests/abc122/tasks/abc122_c)
  - [ ] [ABC 177 C - C - Sum of product of pairs](https://atcoder.jp/contests/abc177/editorial/88)
  - [ ] [AGC 023 A - Zero-Sum Ranges](https://atcoder.jp/contests/agc023/tasks/agc023_a)
  - [ ] [ABC 005 D - おいしいたこ焼きの焼き方](https://atcoder.jp/contests/abc005/tasks/abc005_4)

[しゃくとり法 (尺取り法) の解説と、それを用いる問題のまとめ](https://qiita.com/drken/items/ecd1a472d3a0e7db8dce)

- しゃくとり法
  - [ ] [AOJ Course The Number of Windows](http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_3_C&lang=jp)
  - [ ] [POJ 3061 Subsequence](http://poj.org/problem?id=3061)
  - [ ] [ABC 032 C 列](https://atcoder.jp/contests/abc032/tasks/abc032_c)
  - [ ] [ABC 038 C 単調増加](https://beta.atcoder.jp/contests/abc038/tasks/abc038_c)
  - [ ] [ARC 022 B 細長いお菓子](https://beta.atcoder.jp/contests/arc022/tasks/arc022_2)
  - [ ] [ABC 098 D Xor Sum 2](https://beta.atcoder.jp/contests/abc098/tasks/arc098_b)
  - [ ] [ABC 017 D サプリメント](https://beta.atcoder.jp/contests/abc017/tasks/abc017_4)
  - [ ] [JOI 2013 本選 C バームクーヘン](http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=0600)
  - [ ] [JOI 2017 予選 E L番目のK番目の数](http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=0646)

## 全探索

- [たのしい探索アルゴリズムの世界【前編：全探索、bit全探索から半分全列挙まで】](https://qiita.com/e869120/items/25cb52ba47be0fd418d6#2-%E3%81%99%E3%81%B9%E3%81%A6%E3%81%AE%E5%9F%BA%E6%9C%AC%E5%85%A8%E6%8E%A2%E7%B4%A2)
- [ビット演算 (bit 演算) の使い方を総特集！ 〜 マスクビットから bit DP まで 〜](https://qiita.com/drken/items/7c6ff2aa4d8fce1c9361#6-bit-%E5%85%A8%E6%8E%A2%E7%B4%A2)
- [[AtCoder] AC - 3.05.ビット演算](https://atcoder.jp/contests/apg4b/tasks/APG4b_ac?lang=ja)

問題集

- 全探索
  - [x] [ABC 081 B - Shift Only](https://atcoder.jp/contests/abc081/tasks/abc081_b)
  - [x] [ABC 102 B - Maximum Difference](https://atcoder.jp/contests/abc102/tasks/abc102_b)
  - [x] [ABC 113 B - Palace](https://atcoder.jp/contests/abc113/tasks/abc113_b)
  - [x] [ABC 072 B - OddString](https://atcoder.jp/contests/abc072/tasks/abc072_b)
  - [x] [ABC 053 B - A to Z String](https://atcoder.jp/contests/abc053/tasks/abc053_b)
  - [x] [ABC 095 B - Bitter Alchemy](https://atcoder.jp/contests/abc095/tasks/abc095_b)
  - [x] [ABC 133 B - Good Distance](https://atcoder.jp/contests/abc133/tasks/abc133_b)
  - [x] [ABC 175 B - Making Triangle](https://atcoder.jp/contests/abc175/tasks/abc175_b)
  - [x] [ABC 144 B - 81](https://atcoder.jp/contests/abc144/tasks/abc144_b)
  - [x] [ABC 150 B - Count ABC](https://atcoder.jp/contests/abc150/tasks/abc150_b)
  - [x] [ABC 122 B - ATCoder](https://atcoder.jp/contests/abc122/tasks/abc122_b)
  - [x] [ABC 136 B - Uneven Numbers](https://atcoder.jp/contests/abc136/tasks/abc136_b)
  - [x] [ABC 106 B - 105](https://atcoder.jp/contests/abc106/tasks/abc106_b)
  - [x] [ABC 120 B - K-th Common Divisors](https://atcoder.jp/contests/abc120/tasks/abc120_b)
  - [x] [ABC 105 B - Cakes and Donuts](https://atcoder.jp/contests/abc105/tasks/abc105_b)
  - [x] [ABC 051 B - Sum of Three Integer](https://atcoder.jp/contests/abc051/tasks/abc051_b)
  - [x] [ABC パ研杯2019 C - カラオケ](https://atcoder.jp/contests/pakencamp-2019-day3/tasks/pakencamp_2019_day3_c)
  - [x] [ABC 068 B - Break Numbers](https://atcoder.jp/contests/abc068/tasks/abc068_b)
  - [x] [ABC 157 C - Guess the Number](https://atcoder.jp/contests/abc157/tasks/abc157_c)
  - [x] [ARC 004 A 2点間距離の最大値](https://atcoder.jp/contests/arc004/tasks/arc004_1)
  - [x] [M-SOLUTIONSプロコンオープン2020 B - Magic 2](https://atcoder.jp/contests/m-solutions2020/tasks/m_solutions2020_b)
  - [x] [三井住友信託銀行プログラミングコンテスト2019 B - Tax Rate](https://atcoder.jp/contests/sumitrust2019/tasks/sumitb2019_b)
- 複雑な探索
  - [ ] [ABC 085 C - Otoshidama](https://atcoder.jp/contests/abc085/tasks/abc085_c)
  - [ ] [ABC 112 C - Pyramid](https://atcoder.jp/contests/abc112/tasks/abc112_c)
  - [ ] [ABC 088 C - Takahashi's Information](https://atcoder.jp/contests/abc088/tasks/abc088_c)
  - [ ] [ABC 057 C - Digits in Multiplication](https://atcoder.jp/contests/abc057/tasks/abc057_c)
  - [ ] [ABC 095 C - Half and Half](https://atcoder.jp/contests/abc095/tasks/arc096_a)
  - [ ] [ABC 089 C - March](https://atcoder.jp/contests/abc089/tasks/abc089_c)
  - [ ] [三井住友信託銀行プログラミングコンテスト2019 C - 100 to 105](https://atcoder.jp/contests/sumitrust2019/tasks/sumitb2019_c)
  - [ ] [三井住友信託銀行プログラミングコンテスト2019 D - Lucky PIN](https://atcoder.jp/contests/sumitrust2019/tasks/sumitb2019_d)
  - [ ] [JOI 2007 本選 3 - 最古の遺跡](https://atcoder.jp/contes[ts/joi2007ho/tasks/joi2007ho_c)
  - [ ] [Square869120Contest #6 B - AtCoder Markets](https://atcoder.jp/contests/s8pc-6/tasks/s8pc_6_b)
  - [ ] [JOI 2008 予選 4 - 星座探し](https://atcoder.jp/contests/joi2008yo/tasks/joi2008yo_d)
- bit全探索
  - [ ] [ABC 079 C - Train Ticket](https://atcoder.jp/contests/abc079/tasks/abc079_c)
  - [ ] [ABC 045 C - たくさんの数式](https://atcoder.jp/contests/arc061/tasks/arc061_a)
  - [ ] [ABC 104 C - All Green](https://atcoder.jp/contests/abc104/tasks/abc104_c)
  - [ ] [ARC 029 A - 高橋君とお肉](https://atcoder.jp/contests/arc029/tasks/arc029_1)
  - [ ] [ABC 002 D - 派閥](https://atcoder.jp/contests/abc002/tasks/abc002_4)
  - [ ] [ABC 128 C - Switches](https://atcoder.jp/contests/abc128/tasks/abc128_c)
  - [ ] [ABC 147 C - HonestOrUnkind2](https://atcoder.jp/contests/abc147/tasks/abc147_c)
  - [ ] [ABC 167 C - Skill Up](https://atcoder.jp/contests/abc167/tasks/abc167_c)
  - [ ] [ABC 197 C - ORXOR](https://atcoder.jp/contests/abc197/tasks/abc197_c)
  - [ ] [ABC 080 C - Shopping Street](https://atcoder.jp/contests/abc080/tasks/abc080_c)
  - [ ] [ABC182 C - To 3](https://atcoder.jp/contests/abc182/tasks/abc182_c)
  - [ ] [ABC190 C - Bowls and Dishes](https://atcoder.jp/contests/abc190/tasks/abc190_c)
  - [ ] [JOI 2008 予選 5 - おせんべい](https://atcoder.jp/contests/joi2008yo/tasks/joi2008yo_e)
  - [ ] [Square869120Contest #4 B - Buildings are Colorful!](https://atcoder.jp/contests/s8pc-4/tasks/s8pc_4_b)
- bit全探索 - 拡張
  - [ ] [ABC 031 D - 語呂合わせ](https://atcoder.jp/contests/abc031/tasks/abc031_d)
  - [ ] [M-SOLUTIONS プロコンオープン 2020 E - M's Solution](https://atcoder.jp/contests/m-solutions2020/tasks/m_solutions2020_e)
- 順列全探索
  - [ ] [ABC 145 C - Average Length](https://atcoder.jp/contests/abc145/tasks/abc145_c)
  - [ ] [ABC 150 C - Count Order](https://atcoder.jp/contests/abc150/tasks/abc150_c)
  - [ ] [ABC 054 C - One-stroke Path](https://atcoder.jp/contests/abc054/tasks/abc054_c)
  - [ ] [ABC 183 C - Travel](https://atcoder.jp/contests/abc183/tasks/abc183_c)
  - [ ] [JOI 2009 予選 D カード並べ](https://atcoder.jp/contests/joi2010yo/tasks/joi2010yo_d)
- 再帰
  - [ ] [ABC 165 C - Many Requirements](https://atcoder.jp/contests/abc165/tasks/abc165_c)
  - [ ] [CPSCO2019 Day1 C - Coins](https://atcoder.jp/contests/cpsco2019-s1/tasks/cpsco2019_s1_c)
  - [ ] [パナソニックプログラミングコンテスト 2020 D - String Equivalence](https://atcoder.jp/contests/panasonic2020/tasks/panasonic2020_d)

バチャ

- [全探索](https://kenkoooo.com/atcoder/#/contest/show/05dd88b8-9946-4356-900e-0772486976b4)

## 再帰と分割統治法

- [再帰関数を学ぶと、どんな世界が広がるか](https://qiita.com/drken/items/23a4f604fa3f505dd5ad)

問題集

- [ABC 079 B – Lucas Number](https://math.nakaken88.com/textbook/cp-fibonacci-sequence-and-recursive-function/)
- [ABC 114 C - 755](https://atcoder.jp/contests/abc114/tasks/abc114_c)
- [EDPC A – Frog 1](https://atcoder.jp/contests/dp/tasks/dp_a)
- [ABC 196 D - Hanjo](https://atcoder.jp/contests/abc196/tasks/abc196_d)

## 動的計画法

- [動的計画法（Dynamic Programming）入門](https://algo-logic.info/dynamic-programming/)
- [動的計画法超入門！ Educational DP Contest の A ～ E 問題の解説と類題集](https://qiita.com/drken/items/dc53c683d6de8aeacf5a)
- [Educational DP Contest の F ～ J 問題の解説と類題集](https://qiita.com/drken/items/03c7db44ccd27820ea0d)
- [ゲームを解く！Educational DP Contest K, L 問題の解説](https://qiita.com/drken/items/4e1bcf8413af16cb62da)
- [Educational DP Contestの勧め](https://blog.hamayanhamayan.com/entry/2019/01/12/163853)
- [典型的な DP (動的計画法) のパターンを整理 Part 1 ～ ナップサック DP 編 ～](https://qiita.com/drken/items/a5e6fe22863b7992efdb)

EDPC問題集

- Educational DP Content & Related Problems
  - [x] [EDPC A - Frog1](https://atcoder.jp/contests/dp/tasks/dp_a)
  - [x] [ABC 040 C - 柱柱柱柱柱](https://atcoder.jp/contests/abc040/tasks/abc040_c)
  - [x] [ABC 129 C - Typical Stairs](https://atcoder.jp/contests/abc129/tasks/abc129_c)
  - [ ] [AOJ 0168 観音堂](http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=0168)
  - [x] [EDPC B - Frog2](https://atcoder.jp/contests/dp/tasks/dp_b)
  - [x] [ABC 099 C - Strange Bank](https://atcoder.jp/contests/abc099/tasks/abc099_c)
  - [x] [EDPC C - Vacation](https://atcoder.jp/contests/dp/tasks/dp_c)
  - [x] [EDPC D - Knapsack1](https://atcoder.jp/contests/dp/tasks/dp_d)
  - [x] [TDPC A - コンテスト](https://atcoder.jp/contests/tdpc/tasks/tdpc_contest)
  - [ ] [TDPC D - サイコロ](https://atcoder.jp/contests/tdpc/tasks/tdpc_dice)
  - [ ] [ABC 015 D - 高橋くんの苦悩](https://atcoder.jp/contests/abc015/tasks/abc015_4)
  - [x] [JOI 2010 予選 D - 1年生](https://atcoder.jp/contests/joi2011yo/tasks/joi2011yo_d)
  - [ ] [JOI 2011 予選 D - パスタ](https://atcoder.jp/contests/joi2012yo/tasks/joi2012yo_d)
  - [ ] [JOI 2012 予選 D - 暑い日々](https://atcoder.jp/contests/joi2013yo/tasks/joi2013yo_d)
  - [ ] [JOI 2010 予選 B - 古本屋](https://atcoder.jp/contests/joi2011ho/tasks/joi2011ho2)
  - [ ] [AOJ 2566 Restore Calculation](http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=2566)
  - [x] [EDPC E - Knapsack2](https://atcoder.jp/contests/dp/tasks/dp_e)
  - [x] [ARC 057 B - 高橋君ゲーム](https://atcoder.jp/contests/arc057/tasks/arc057_b)
  - [ ] [AOJ 2263 ファーストアクセプタンス](http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=2263)
  - [ ] [ABC 032 D ナップサック問題](https://atcoder.jp/contests/abc032/tasks/abc032_d)

手法別問題集

- ナップザックDP
  - [ALDS_10_A - フィボナッチ数](http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_10_A&lang=ja)
  - [DPL_1_B - 0,1ナップザック問題](http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_1_B&lang=ja)
  - [DPL_1_C - ナップザック問題](http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_1_C&lang=ja)
  - [DPL_1_A - コイン問題](http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_1_A&lang=ja)
  - [ALDS_10_C - 最長共通部分列](http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_10_C&lang=ja)
  - [JOI 2011 予選 4 - 1 年生](https://atcoder.jp/contests/joi2011yo/tasks/joi2011yo_d)
  - [JOI 2012 予選 4 - パスタ](https://atcoder.jp/contests/joi2012yo/tasks/joi2012yo_d)
  - [OI 2013 予選 4 - 暑い日々](https://atcoder.jp/contests/joi2013yo/tasks/joi2013yo_d)
  - [JOI 2015 予選 4 - シルクロード](https://atcoder.jp/contests/joi2015yo/tasks/joi2015yo_d)
  - [パ研杯2019 D - パ研軍旗](https://atcoder.jp/contests/pakencamp-2019-day3/tasks/pakencamp_2019_day3_d)
  - [AOJ 1167 - ポロック予想](http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=1167&lang=jp)
  - [AOJ 2199 - 差分パルス符号変調](http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=2199&lang=jp)
- 区間DP
  - [ALDS_10_B - 連鎖行列積](http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_10_B&lang=ja)
  - [JOI 2015 本選 2 - ケーキの切り分け 2](https://atcoder.jp/contests/joi2015ho/tasks/joi2015ho_b)
  - [AOJ 1611 ダルマ落とし](http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=1611&lang=jp)
- bit DP
  - [DPL_2_A - 巡回セールスマン問題](http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_2_A&lang=ja)
  - [Square869120Contest #1 G - Revenge of Traveling Salesman Problem](https://atcoder.jp/contests/s8pc-1/tasks/s8pc_1_g)
  - [JOI 2014 予選 4 - 部活のスケジュール表](https://atcoder.jp/contests/joi2014yo/tasks/joi2014yo_d)
  - [JOI 2017 予選 4 - ぬいぐるみの整理](https://atcoder.jp/contests/joi2017yo/tasks/joi2017yo_d)

## 二分探索法

参考資料

- [競技プログラミングにおける二分探索・三分探索問題まとめ](http://hamayanhamayan.hatenablog.jp/entry/2017/07/05/160236)

問題集

- [ALDS_4_B - 二分探索](http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_4_B&lang=ja)
- [JOI 2009 本選 2 - ピザ](https://atcoder.jp/contests/joi2009ho/tasks/joi2009ho_b)
- [ABC 077 C - Snuke Festival](https://atcoder.jp/contests/abc077/tasks/arc084_a)
- [ABC 023 D - 射撃王](https://atcoder.jp/contests/abc023/tasks/abc023_d)
- [ARC 054 B - ムーアの法則](https://atcoder.jp/contests/arc054/tasks/arc054_b)
- [JOI 2008 本選 3 - ダーツ](https://atcoder.jp/contests/joi2008ho/tasks/joi2008ho_c)

## 貪欲法

## 配列、連結リスト、ハッシュテーブル

## スタック、キュー

## グラフと木

## Union-Find

- [ARC032 B - 道路工事](https://atcoder.jp/contests/arc032/tasks/arc032_2)

## グラフ探索

参考資料

- [DFS (深さ優先探索) 超入門！ 〜 グラフ・アルゴリズムの世界への入口 〜【前編】](https://qiita.com/drken/items/4a7869c5e304883f539b)
- [DFS (深さ優先探索) 超入門！ 〜 グラフ・アルゴリズムの世界への入口 〜【後編】](https://qiita.com/drken/items/a803d4fc4a727e02f7ba)
- [深さ優先探索(Depth First Search)の基本](https://algo-logic.info/dfs/)
- [LeetCode DFS](https://leetcode.com/tag/depth-first-search/)
- [Pythonでアルゴリズムの勉強~全探索](https://www.tsumar.com/blog/algorithm_whole_search/)

問題集

- 深さ優先探索
  - [ ] [ATC 001 A 深さ優先探索](https://atcoder.jp/contests/atc001/tasks/dfs_a)
  - [ ] [ALDS_11_B - 深さ優先探索](http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_11_B)
  - [ ] [AOJ How Many Islands?](http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=1160&lang=jp)
  - [ ] [AOJ 1160 - 島はいくつある？](http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=1160&lang=jp)
  - [ ] [ABC 138 D - Ki](https://atcoder.jp/contests/abc138/tasks/abc138_d)
  - [ ] [ARC 031 B 埋め立て](https://atcoder.jp/contests/arc031/tasks/arc031_2)
  - [ ] [ARC 037 B バウムテスト](https://atcoder.jp/contests/arc037/tasks/arc037_b)
  - [ ] [JOI 2009 予選 4 - 薄氷渡り](https://atcoder.jp/contests/joi2009yo/tasks/joi2009yo_d)
  - [ ] [ABC 015 C - 高橋くんのバグ探し](http://abc015.contest.atcoder.jp/tasks/abc015_3)
  - [ ] [ABC 029 C - Brute-force Attack](https://atcoder.jp/contests/abc029/tasks/abc029_c)
  - [ ] [ABC 054 C - One-stroke Path](https://atcoder.jp/contests/abc054/tasks/abc054_c)
  - [ ] [ABC 067 D - Fennec VS. Snuke](https://atcoder.jp/contests/abc067/tasks/arc078_b)
  - [ ] [ABC 126 D - Even Relation](https://atcoder.jp/contests/abc126/tasks/abc126_d)
  - [ ] [CODE FESTIVAL 2017 qualB C - 3 Steps](https://atcoder.jp/contests/code-festival-2017-qualb/tasks/code_festival_2017_qualb_c)

参考資料

- [BFS (幅優先探索) 超入門！ 〜 キューを鮮やかに使いこなす 〜](https://qiita.com/drken/items/996d80bcae64649a6580)

問題集

- 幅優先探索
  - [ ] [ALDS_11_C - 幅優先探索](http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_11_C&lang=ja)
  - [ ] [ABC 007 C - 幅優先探索](https://atcoder.jp/contests/abc007/tasks/abc007_3)
  - [ ] [JOI 2011 予選 5 - チーズ](https://atcoder.jp/contests/joi2011yo/tasks/joi2011yo_e)
  - [ ] [JOI 2012 予選 5 - イルミネーション](https://atcoder.jp/contests/joi2012yo/tasks/joi2012yo_e)
  - [ ] [AOJ 1166 - 迷図と命ず](http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=1166&lang=jp)
  - [ ] [ABC 088 D - Grid Repainting](https://atcoder.jp/contests/abc088/tasks/abc088_d)
  - [ ] [AGC 033 A - Darker and Darker](https://atcoder.jp/contests/agc033/tasks/agc033_a)
  - [ ] [ARC 005 C 器物損壊！高橋君](https://atcoder.jp/contests/arc005/tasks/arc005_3)
  - [ ] [AOJ 0503 Cup](http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=0503)

## 最短路

## 最小全域木

## ネットワークフロー

## 参考資料

- [AtCoder Beginners Selection](https://atcoder.jp/contests/abs)
- [競プロ典型 90 問](https://atcoder.jp/contests/typical90)
- [AtCoder に登録したら次にやること ～ これだけ解けば十分闘える！過去問精選 10 問 ～](https://qiita.com/drken/items/fd4e5e3630d0f5859067)
- [AtCoder 版！蟻本 (初級編)](https://qiita.com/drken/items/e77685614f3c6bf86f44)
- [レッドコーダーが教える、競プロ・AtCoder上達のガイドライン【初級編：競プロを始めよう】](https://qiita.com/e869120/items/f1c6f98364d1443148b3#1-6-%E8%8C%B6%E8%89%B2%E3%82%B3%E3%83%BC%E3%83%80%E3%83%BC%E3%81%AB%E3%81%AA%E3%82%8B%E3%81%9F%E3%82%81%E3%81%AE%E3%82%AC%E3%82%A4%E3%83%89%E3%83%A9%E3%82%A4%E3%83%B3)
- [レッドコーダーが教える、競プロ・AtCoder上達のガイドライン【中級編：目指せ水色コーダー！】](https://qiita.com/e869120/items/eb50fdaece12be418faa#2-2-%E6%B0%B4%E8%89%B2%E3%82%B3%E3%83%BC%E3%83%80%E3%83%BC%E3%81%AB%E3%81%AA%E3%82%8B%E3%81%9F%E3%82%81%E3%81%AE%E3%82%AC%E3%82%A4%E3%83%89%E3%83%A9%E3%82%A4%E3%83%B3)
- [https://github.com/drken1215/book_algorithm_solution](https://github.com/drken1215/book_algorithm_solution)
- [あのアルゴリズムはどこ？　Pythonを使用してAtCoderの緑色や水色を目指す方に、30以上のアルゴリズムスニペットと100問以上の問題（ACコード付き）を紹介！](https://qiita.com/H20/items/1a066e242815961cd043)
- [RustCoder ―― AtCoder と Rust で始める競技プログラミング入門](https://zenn.dev/toga/books/rust-atcoder)
- [アルゴリズムビジュアル大辞典](https://yutaka-watanobe.github.io/star-aida/books/)
- [AtCoderの問題を分類しました](https://qiita.com/KoyanagiHitoshi/items/32dc42d8c5ee75339e54#%E7%AC%AC7%E7%AB%A0-%E3%83%87%E3%83%BC%E3%82%BF%E6%A7%8B%E9%80%A0%E8%A6%B3%E7%82%B9)
- [AtCoder 問題カテゴリー分類](https://qiita.com/c-yan/items/56a051d826b873b4f78d)
