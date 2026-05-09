# about

jsonを指定して下記グラフを画像として出力するプロダクトを開発する。

+ 折れ線
+ 棒グラフ
+ 散布図

コンソールのパラメタとして下記を指定

+ 出力先(--output <string>| -o <string>)
  + デフォルト: ./output/image.png
+ 画像サイズ( --size <int>| -s <int>)
  + デフォルト: 256x256
+ 入力(--input <string:path> | -i <string:path>)
  + 指定されていない場合は標準入力を利用する
+ グラフタイプ(--type <string:graphtype> | -t <string:graphtype>)
  + デフォルト: line
  + graphtype: line, bar, scatter


rustで開発する。
