講義(Lecture) 9:  物理層とデータリンク層(I)
INIAD学内限定配布
(＊出典等も教育目的につき、簡潔化のため省略しています)
Limited distribution inside INIAD
(For educational purposes references are omitted for simplicity)

----------------
1. 物理層 Physical Layer
Copyright © 2024 by INIAD
2
実際に通信が行われている物理層の仕組みを学習します

----------------
OSI リファレンス・モデル
Copyright © 2024 by INIAD
3
Physical Layer（物理層）
(必ずしも信頼性があるわけでない)直接的なポイントツーポイントデータ接続の提供
接続デバイスをネットワークに接続するハードウェアの電気、機械、機能、および手順の仕様の定義 
ビットが電圧に（電圧がビットに）変換される部分 
データ単位：ビット 
例）コネクタのサイズと形状、ピンの数、使用可能な
　信号（signals)、IEEE 802.3、IEEE 802.11 
Application
Presentation
Session
Transport 
Network
Data Link
Physical
OSI Model
Physical
Interface
Media (UTP, fiber, wireless, etc.)
復習

----------------
実際の通信（実際の送受信） 
Copyright © 2024 by INIAD
4
メディアによるコミュニケーション 
非シールドツイストペア（Unshielded Twisted Pair (UTP) ）
 8本（二本1組の4組)の銅線でできている
全二重（双方向） 
簡単に曲げられるため設置し易い
基本的にどこでもインストールされている 
ファイバ （Fiber）
高速伝送が可能 
曲がりにくく、設置に制限がある
高価 
近年インストールが増えている 
空気 （Air）
物理配線が不要
簡単で迅速なネットワークの配置が可能
IoTは 特にこのメディアを活用している 

----------------
Copyright © 2024 by INIAD
5
インタフェースとの通信 
コンピュータが限られたエリア内で相互接続されているローカルエリアネットワーク（LAN: local area network）内: 
インタフェース - NIC（ネットワークインタフェースカード）



長距離および異なるネットワークアーキテクチャ間でデータが送信される広域ネットワーク（WAN: wide area network）内: 
インタフェース - NIC、USBポート、通信ポート、 
DCE（データ回線終端装置） - 端末と通信網 の間に位置し、信号変換、符号化等が行われる機器（例: モデム等）
実際の通信（実際の送受信） 
Interface
Media (UTP, fiber, wireless, etc.)
Interface
Modem
Media (WAN cable)

----------------
ちなみに
Copyright © 2024 by INIAD
6
←これは「インタフェース」です （以降で，「ポート」という場合もあります）
TCPのポート番号，の「ポート」とは異なるので注意してください（文脈から分かると思いますが念のため）

----------------
ちなみに
Copyright © 2024 by INIAD
6
←これは「インタフェース」です （以降で，「ポート」という場合もあります）
TCPのポート番号，の「ポート」とは異なるので注意してください（文脈から分かると思いますが念のため）

----------------
ちなみに
Copyright © 2024 by INIAD
6
←これは「インタフェース」です （以降で，「ポート」という場合もあります）
TCPのポート番号，の「ポート」とは異なるので注意してください（文脈から分かると思いますが念のため）

----------------
Signal (信号) 
Copyright © 2024 by INIAD
7
Signalの問題：
Attenuation(減衰)
Noise and interference(ノイズと干渉)
Collision(衝突)

----------------
Copyright © 2024 by INIAD
8
トラベルの距離が増加するにつれてSignal強度が減少する
Signal Attenuation (減衰) 
Signal strength
Signal attenuation

----------------
Signal noise & interference (ノイズと干渉)
Copyright © 2024 by INIAD
9
Signal noise & interferenceによるシグナル変形
近くの類似信号（クロストーク） 
熱雑音（発熱体から） 
高電圧電気資源 
Signal deformation（シグナルの変形）

----------------
Signal collision (衝突)  
Copyright © 2024 by INIAD
10
Signalの衝突はSignalの変形をもたらす
共有メディア上のSignalの衝突
Signal deformation （Signalの変形）

----------------
Copyright © 2024 by INIAD
11

解決策：全二重システム(Fully-duplex system) 

専用メディアの場合は衝突なし 
Signal の衝突は信号の変形をもたらす 
共有メディア上のSignalの衝突 
Signal collision (衝突) 

----------------
Copyright © 2024 by INIAD
12
Signalを増幅するデバイス。衝突ドメイン＊を拡張し、複数のコンピュータを接続するデバイス
Signalは遠ざかるにつれて弱くなる 
Signal強度を増幅するためのハブを挿入する
Hub (ハブ)
Signal strength
Hub
＊衝突ドメイン：ハブで繋がっているコンピュータに同時に信号を送った時に衝突が起こりうる範囲

----------------
Hub (ハブ)
Copyright © 2024 by INIAD
Floodingとは – signalを受信したポートを除いた全てのポートにsignalをブロードキャストする。これをフラッディングと言う 
Hub
13

----------------
Hubのいいところと良くないところ
Copyright © 2024 by INIAD
14
衝突ドメインを拡張することによって、より離れたデバイスに信号が届く
ハブによって直接接続されているコンピュータが多すぎると、衝突ドメイン（衝突が起こりうる範囲）が増加する。よってネットワーク内が混雑しやすくなる。
衝突ドメインは，頻繁な衝突を避けられる程度に小さくあるべき！
Hub

----------------
2.データリンク層 Data Link Layer
Copyright © 2024 by INIAD
15
通信手段を提供するデータリンク層の仕組みを学習します

----------------
OSI リファレンス・モデル
Copyright © 2024 by INIAD
16
Data Link Layer (データリンク層）
直接ポイントツーポイント(隣接機器間）の信頼できるデータ接続の提供 
データ単位：ビット/フレーム 
例）
IEEE802.11 (無線Local Area Network (LAN)を実装するためのメディア
　アクセスコントロール(MAC)および物理層のIEEE標準)
IEEE 802.3（物理層および有線イーサネットのMACを定義するIEEE標準の集合） 
Application
Presentation
Session
Transport 
Network
Data Link
Physical
OSI Model
Data Link
*IEEE (The Institute of Electrical and Electronics Engineers)　- 通信技術等の標準化に携わっているアメリカ合衆国に本部を持つ電気電子学会
Switch
例） Ethernet
ポイントツーポイント通信
復習

----------------
データリンク層 (Data Link Layer)
Copyright © 2024 by INIAD
17

インターネットはネットワークのネットワーク
様々な通信技術が同時に存在している
Ethernet, Wi-Fi, satellite network等
個々の通信技術を提供するのがデータリンク層
Switch
E.g.） Ethernet 通信

----------------
データリンク層 (Data Link Layer)の主な役割
Copyright © 2024 by INIAD
18

ホップごとのフレームの確実な配信 

フレーミング (Framing)
転送 (Forwarding)
Switch

----------------
フレーミング (Framing)
Copyright © 2024 by INIAD
19
送信側が受信側にとって意味のある一連のビットを送信する方法を提供する
2つのデバイス間のポイントツーポイント間でデータがビットストリームとして伝送される
識別可能な情報ブロックに囲まれなければならない
例）Ethernetでのフレーミング 
Preamble（プリアンブル）とStart Frame Delimiter (SFD) を使用
Preamble: 送信側と受信側のNICが時間の基準となる信号（クロック）の同期をとるために使われる 56ビットの１と０のくりかえしのシグナル 
SFD: 受信側のNICがフレームの始まりを検知するための8ビットのシグナル（10101011）
Ethernet Frame
Preamble (56 bits)
SFD (8 bits)
10101010 10101010 …10101010   10101011 
クロックの同期に使用!
受信機がフレームの始まりを検出するために使用！

----------------
フレーミング (Framing)
Copyright © 2024 by INIAD
19
送信側が受信側にとって意味のある一連のビットを送信する方法を提供する
2つのデバイス間のポイントツーポイント間でデータがビットストリームとして伝送される
識別可能な情報ブロックに囲まれなければならない
例）Ethernetでのフレーミング 
Preamble（プリアンブル）とStart Frame Delimiter (SFD) を使用
Preamble: 送信側と受信側のNICが時間の基準となる信号（クロック）の同期をとるために使われる 56ビットの１と０のくりかえしのシグナル 
SFD: 受信側のNICがフレームの始まりを検知するための8ビットのシグナル（10101011）
Ethernet Frame
Preamble (56 bits)
SFD (8 bits)
10101010 10101010 …10101010   10101011 
クロックの同期に使用!
受信機がフレームの始まりを検出するために使用！

----------------
フレーミング (Framing)
Copyright © 2024 by INIAD
19
送信側が受信側にとって意味のある一連のビットを送信する方法を提供する
2つのデバイス間のポイントツーポイント間でデータがビットストリームとして伝送される
識別可能な情報ブロックに囲まれなければならない
例）Ethernetでのフレーミング 
Preamble（プリアンブル）とStart Frame Delimiter (SFD) を使用
Preamble: 送信側と受信側のNICが時間の基準となる信号（クロック）の同期をとるために使われる 56ビットの１と０のくりかえしのシグナル 
SFD: 受信側のNICがフレームの始まりを検知するための8ビットのシグナル（10101011）
Ethernet Frame
Preamble (56 bits)
SFD (8 bits)
10101010 10101010 …10101010   10101011 
クロックの同期に使用!
受信機がフレームの始まりを検出するために使用！

----------------
フレーミング (Framing)
Copyright © 2024 by INIAD
19
送信側が受信側にとって意味のある一連のビットを送信する方法を提供する
2つのデバイス間のポイントツーポイント間でデータがビットストリームとして伝送される
識別可能な情報ブロックに囲まれなければならない
例）Ethernetでのフレーミング 
Preamble（プリアンブル）とStart Frame Delimiter (SFD) を使用
Preamble: 送信側と受信側のNICが時間の基準となる信号（クロック）の同期をとるために使われる 56ビットの１と０のくりかえしのシグナル 
SFD: 受信側のNICがフレームの始まりを検知するための8ビットのシグナル（10101011）
Ethernet Frame
Preamble (56 bits)
SFD (8 bits)
10101010 10101010 …10101010   10101011 
クロックの同期に使用!
受信機がフレームの始まりを検出するために使用！

----------------
Forwarding（転送）
Copyright © 2024 by INIAD
20
２種類のアクセスリンク：ブロードキャストリンク とポイントツーポイントリンク

1. ポイントツーポイントリンクは、通信に参加する2者だけが使うメディアを使用（以下、専用メディア。例：ギガビットイーサネット）
2. ブロードキャストリンクは、すべての通信参加者が１つのメディアを共有して使用（以下、共有メディア。例：Wi-Fi） 


----------------
Copyright © 2024 by INIAD
21
1. ポイントツーポイント (Point-to-point) リンクは専用メディアを使用 
一対一のリンク
回線あたりのコストが高くなる
例）スイッチングイーサネット (Switching Ethernet), PPP (Point-to-Point Protocol)
Forwarding（転送）
Po
i
n
t
-t
o
-p
o
i
n
t
 
Po
i
n
t
-t
o
-p
o
i
n
t
 
（受信先がいつも１台なので識別が容易）
（端末デバイスと端末デバイスの間）
（スイッチとスイッチの間）
（端末デバイスとスイッチの間）
Po
i
n
t
-t
o
-p
o
i
n
t
 

----------------
Copyright © 2024 by INIAD
22
2. ブロードキャストリンクは共有メディアを使用
共有型リンク
回線あたりのコストが下がる
例）ブロードキャスト（802.11ワイヤレスLANなど） 
Forwarding（転送）
（複数の受信者が存在している環境のため識別が必要）
Wi-Fi アクセスポイント
端末デバイス
802.11 ワイヤレス LAN

----------------
3. イーサネット Ethernet
Copyright © 2024 by INIAD
23

----------------
イーサネット（Ethernet） 
1980年代に商業的に導入された有線ネットワーク技術 
ローカルエリアネットワーク（LAN）のデファクトスタンダード 
物理層およびデータリンク層のメディアアクセス制御（MAC＝Media Access Control）のための、IEEE 802.3規格に基づいたスタンダード
２つのイーサネット
イーサネット：共有メディアを使用。複数のデバイスが同時に（あるいはほぼ同時に）通信したい時に信号が衝突してしまい、通信の効率が悪くなるという欠点がある（初代からのタイプ）
スイッチングイーサネット：専用メディアを使用 （現代普及しているタイプ）
Copyright © 2024 by INIAD
24
まずはイーサネットについて理解を深める
(後ほどスイッチングイーサネットについて)

----------------
Copyright © 2024 by INIAD
25
Frame format
Dst Address (48 bits): 宛先MACアドレス*
Src Address (48 bits): 送信元MACアドレス
Type: ネットワーク層のプロトコルタイプ
Payload: ネットワーク層以上の全てを含む
FCS(Frame Check Sequence): エラーの検出に使われる
*Media Access Control (MAC)アドレス：ネットワーク内をデータリンク層で通信するためにネットワークインタフェースに割り当てられたユニークな物理アドレス
［Dst Address］ | ［Src Address] ｜［Type] ｜［Payload］｜［FCS］
イーサネット（Ethernet）
（FCSはWiresharkでキャプチャされない）

----------------
イーサネット（Ethernet）
Copyright © 2024 by INIAD
26
CSMA/CDでのブロードキャストによるメディアアクセス 　
CSMA(Carrier Sense Multiple Access)
送信する前に聞く（チャンネルが静かか確かめる） 
CD(Collision Detection)
衝突が検出された場合に送信を終了する 
許容可能な送信レートをみつけるために、送信試行のレートを乗算的に減少させるExponential Backoff (指数バックオフ)アルゴリズムを組み込む
 Exponential Backoff （指数バックオフ） - 同じフレームの送信試行で衝突が検出されるたびに、送信者はしばらくの間、フレーム再送信からバックオフします。そのバックオフ期間は送信者が同じフレームの送信試行でフレーム衝突を繰り返すと指数関数的に増加する。

----------------
CSMA: メディアが使用されていなければ（静かであれば）フレーム全体を送信する 
Copyright © 2024 by INIAD
27

----------------
CD(衝突検出) :衝突検出後瞬時に送信を終了する  
Copyright © 2024 by INIAD
28
無駄な送信を最小限に抑える！☺ 
Station A
Station B
Time
Station C
X
X
Collision detected
X

----------------
CD(衝突検出) :衝突検出後瞬時に送信を終了する  
Copyright © 2024 by INIAD
28
無駄な送信を最小限に抑える！☺ 
Station A
Station B
Time
Station C
X
X
Collision detected
X

----------------
CD(衝突検出) :衝突検出後瞬時に送信を終了する  
Copyright © 2024 by INIAD
28
無駄な送信を最小限に抑える！☺ 
Station A
Station B
Time
Station C
X
X
Collision detected
X

----------------
CD (もし衝突検知（CD)がなかったら…)
Copyright © 2024 by INIAD
29
送信者がフレームの全てを送り出すまで無駄な送信が続く！☹ 

----------------
CD (もし衝突検知（CD)がなかったら…)
Copyright © 2024 by INIAD
29
送信者がフレームの全てを送り出すまで無駄な送信が続く！☹ 

----------------
CD (もし衝突検知（CD)がなかったら…)
Copyright © 2024 by INIAD
29
送信者がフレームの全てを送り出すまで無駄な送信が続く！☹ 

----------------
CD (もし衝突検知（CD)がなかったら…)
Copyright © 2024 by INIAD
29
送信者がフレームの全てを送り出すまで無駄な送信が続く！☹ 

----------------
イーサネット（Ethernet） 
Copyright © 2024 by INIAD
30
CSMA/CDでのブロードキャストによるメディアアクセス 
メディアアクセスのPseudocode(擬似コード):
While (送信するフレームがあるとき) {
	If (channel がサイレント（静か）であったら){
	   フレームを送信する
	    
	   If（フレーム全体を送信し終わる前に衝突が起きたら）	
		即座に現時点の送信をやめて指数バックオフを実施
	} 
	Else if (channelが混雑していたら)
		送信するのを後にする
}
バイナリ指数バックオフ - 詳細は次頁にて
アダプタはネットワーク層からデータを取得し、データリンク層フレームを準備

----------------
イーサネット（Ethernet） 
Copyright © 2024 by INIAD
30
CSMA/CDでのブロードキャストによるメディアアクセス 
メディアアクセスのPseudocode(擬似コード):
While (送信するフレームがあるとき) {
	If (channel がサイレント（静か）であったら){
	   フレームを送信する
	    
	   If（フレーム全体を送信し終わる前に衝突が起きたら）	
		即座に現時点の送信をやめて指数バックオフを実施
	} 
	Else if (channelが混雑していたら)
		送信するのを後にする
}
バイナリ指数バックオフ - 詳細は次頁にて
アダプタはネットワーク層からデータを取得し、データリンク層フレームを準備

----------------
イーサネット（Ethernet） 
Copyright © 2024 by INIAD
30
CSMA/CDでのブロードキャストによるメディアアクセス 
メディアアクセスのPseudocode(擬似コード):
While (送信するフレームがあるとき) {
	If (channel がサイレント（静か）であったら){
	   フレームを送信する
	    
	   If（フレーム全体を送信し終わる前に衝突が起きたら）	
		即座に現時点の送信をやめて指数バックオフを実施
	} 
	Else if (channelが混雑していたら)
		送信するのを後にする
}
バイナリ指数バックオフ - 詳細は次頁にて
アダプタはネットワーク層からデータを取得し、データリンク層フレームを準備

----------------
イーサネット（Ethernet） 
Copyright © 2024 by INIAD
30
CSMA/CDでのブロードキャストによるメディアアクセス 
メディアアクセスのPseudocode(擬似コード):
While (送信するフレームがあるとき) {
	If (channel がサイレント（静か）であったら){
	   フレームを送信する
	    
	   If（フレーム全体を送信し終わる前に衝突が起きたら）	
		即座に現時点の送信をやめて指数バックオフを実施
	} 
	Else if (channelが混雑していたら)
		送信するのを後にする
}
バイナリ指数バックオフ - 詳細は次頁にて
アダプタはネットワーク層からデータを取得し、データリンク層フレームを準備

----------------
Binary Exponential Backoff
Copyright © 2024 by INIAD
31
バイナリ指数バックオフ - 送信するフレームを持つ各ノードは、(最大15回)ランダムな間隔で待機する：
1. まず、n回衝突したときには、Kの値を{0, 1, 2, …, 2n-1} からランダムに選ぶ
ただし、11回目以降はnの値を大きくせずに{0, 1, 2, …, 2n-1}（要は{0, 1, 2, …, 210-1}）の範囲からランダムに選ぶ
2. K * 512ビット時間待つ
（512ビットをイーサネットで送信するのに必要な時間のK倍待つ）
n回目の衝突時に、各ノードは0、1、2、…、または2n-1のいずれかをランダムに選択する。すなわち：
最初の衝突時に、各ノードはランダムに0または1を選択
2回目の衝突時に、各ノードは0、1、2、または3のいずれかをランダムに選択
3回目の衝突時に、各ノードは0、1、2、3、4、5、 6、または7のいずれかをランダムに選択
等々
そして、K * 512ビットの時間待つ：例えば、ノードがK = 2を選択した場合、次の再送信までに1024ビット時間待機
各ノードがランダムではなく一定の（同じ）時間待機して再送信するとどうなる？

----------------
Binary Exponential Backoff
Copyright © 2024 by INIAD
31
バイナリ指数バックオフ - 送信するフレームを持つ各ノードは、(最大15回)ランダムな間隔で待機する：
1. まず、n回衝突したときには、Kの値を{0, 1, 2, …, 2n-1} からランダムに選ぶ
ただし、11回目以降はnの値を大きくせずに{0, 1, 2, …, 2n-1}（要は{0, 1, 2, …, 210-1}）の範囲からランダムに選ぶ
2. K * 512ビット時間待つ
（512ビットをイーサネットで送信するのに必要な時間のK倍待つ）
n回目の衝突時に、各ノードは0、1、2、…、または2n-1のいずれかをランダムに選択する。すなわち：
最初の衝突時に、各ノードはランダムに0または1を選択
2回目の衝突時に、各ノードは0、1、2、または3のいずれかをランダムに選択
3回目の衝突時に、各ノードは0、1、2、3、4、5、 6、または7のいずれかをランダムに選択
等々
そして、K * 512ビットの時間待つ：例えば、ノードがK = 2を選択した場合、次の再送信までに1024ビット時間待機
各ノードがランダムではなく一定の（同じ）時間待機して再送信するとどうなる？

----------------
Binary Exponential Backoff
Copyright © 2024 by INIAD
31
バイナリ指数バックオフ - 送信するフレームを持つ各ノードは、(最大15回)ランダムな間隔で待機する：
1. まず、n回衝突したときには、Kの値を{0, 1, 2, …, 2n-1} からランダムに選ぶ
ただし、11回目以降はnの値を大きくせずに{0, 1, 2, …, 2n-1}（要は{0, 1, 2, …, 210-1}）の範囲からランダムに選ぶ
2. K * 512ビット時間待つ
（512ビットをイーサネットで送信するのに必要な時間のK倍待つ）
n回目の衝突時に、各ノードは0、1、2、…、または2n-1のいずれかをランダムに選択する。すなわち：
最初の衝突時に、各ノードはランダムに0または1を選択
2回目の衝突時に、各ノードは0、1、2、または3のいずれかをランダムに選択
3回目の衝突時に、各ノードは0、1、2、3、4、5、 6、または7のいずれかをランダムに選択
等々
そして、K * 512ビットの時間待つ：例えば、ノードがK = 2を選択した場合、次の再送信までに1024ビット時間待機
各ノードがランダムではなく一定の（同じ）時間待機して再送信するとどうなる？

----------------
4. Switching Ethernet
Copyright © 2024 by INIAD
32

----------------
Switching Ethernet 
全二重 (Fully duplex) (スイッチの使用） 
衝突ドメインがないため同時に送受信できる  
Copyright © 2024 by INIAD
33
Po
i
n
t
-t
o
-p
o
i
n
t
 
Layer 2 Switch:
(単にスイッチとも呼びます)
Layer 2 Switch: データリンク層で動作し，MACアドレスを使用してフレームが転送されるパスを決定するネットワークスイッチまたはデバイスのこと

----------------
Switching Ethernet 
Switching Ethernetを支える技術: 
1. Buffering(バッファリング)   
2. MAC Address Filtering(MACアドレスフィルタリング) 

Copyright © 2024 by INIAD
34

----------------
Switching Ethernet 
Switching Ethernetを支える技術: 
1. Buffering(バッファリング)   
複数のポートに同時に（あるいはほぼ同時）に同じポートを介して転送すべきフレームが届いても、それらを同時に送信することはできない。
過剰なフレームをバッファに一時的に格納することをBuffering(バッファリング)と呼ぶ
2. MAC Address Filtering(MACアドレスフィルタリング) 
Copyright © 2024 by INIAD
35

----------------
Buffering (バッファリング)
過剰なフレームをバッファに一時的に格納する 
格納されたフレームは、リンクが利用可能になると順次転送される
    (E.g., 例えば、現在送信されているフレームがその送信を完了した後)
Copyright © 2024 by INIAD
36
Buffer
Port #1
Port #2
Port #3
Port #4

----------------
Buffering (バッファリング)
バッファがいっぱいになるとどうなる？ 
IEEE 802.3x (for Full-duplex switch)
受信者はPause frame（ポーズフレーム）を送信し送信者にスローダウンを求める 
注：ポーズフレームは、隣接する2つのデバイス間で発生する (例えば, スイッチとスイッチの間や，ホストとスイッチの間)
Copyright © 2024 by INIAD
37

----------------
半二重イーサネットもまたバッファリングを実装します 
バッファが一杯になった場合どうなるか(半二重イーサネットの場合 )
Back pressure (背圧機構) - 受信機は、意図的に衝突を引き起こすために送信機に妨害信号 (Jamming signals) を送る 






 
Buffering (半二重イーサネット用 )
Copyright © 2024 by INIAD
38
1．バッファがいっぱいになる
注：妨害信号は、隣接する2つのデバイス間で発生しています 
２．妨害信号を送信する

----------------
Switching Ethernet 
Switching Ethernetを支える技術: 
1. Buffering(バッファリング)
2.  MAC Address Filtering(MACアドレスフィルタリング)
フレームの行き先がわかる時はその送信に必要なインタフェースのみから送信し、わからない時だけブロードキャスト送信する仕組み
余分な通信は控えるが、個々のスイッチがフレームの宛先への経路がわからない状況では全ての隣接スイッチにブロードキャストしてフレームが宛先に届くようにする
Copyright © 2024 by INIAD
39
要は，
フレームをどのインタフェース（ポート）に転送すべきか，フィルタすべきかを判断すること，です


----------------
Switching Ethernet 
Switching Ethernetを支える技術: 
1. Buffering(バッファリング)
2.  MAC Address Filtering(MACアドレスフィルタリング)
フレームの行き先がわかる時はその送信に必要なインタフェースのみから送信し、わからない時だけブロードキャスト送信する仕組み
余分な通信は控えるが、個々のスイッチがフレームの宛先への経路がわからない状況では全ての隣接スイッチにブロードキャストしてフレームが宛先に届くようにする
Copyright © 2024 by INIAD
39
要は，
フレームをどのインタフェース（ポート）に転送すべきか，フィルタすべきかを判断すること，です


----------------
Switching Ethernet 
Switching Ethernetを支える技術: 
1. Buffering(バッファリング)
2.  MAC Address Filtering(MACアドレスフィルタリング)
フレームの行き先がわかる時はその送信に必要なインタフェースのみから送信し、わからない時だけブロードキャスト送信する仕組み
余分な通信は控えるが、個々のスイッチがフレームの宛先への経路がわからない状況では全ての隣接スイッチにブロードキャストしてフレームが宛先に届くようにする
Copyright © 2024 by INIAD
39
要は，
フレームをどのインタフェース（ポート）に転送すべきか，フィルタすべきかを判断すること，です


----------------
MAC Address Filtering  
Self Learning (自己学習)とフォワーディング (転送) を使用
スイッチが受信した各フレームに対して、 送信元MACアドレスと、それが到着したポート(受信ポート)を学習テーブルのエントリに記入し、以降のフレームを転送するためにその知識を使用する
フレームを既知のポートに転送するか、不明な場合はフラッディングする(受信ポート以外のすべてのポートにフレームを転送) 
Pseudocode:
Initial state（初期状態）：学習テーブルは空  For (フレームが到着するごとに) {     If　(送信元情報が、テーブル内で見つかった)          テーブル内のフレーム到着の時刻を更新する     Else //(送信元情報が、テーブル内で見つからなかった場合){         テーブル内に以下を登録する:              ‐送信元MACアドレス              ‐フレーム到着のポート番号              ‐フレーム到着の時刻     }
        If（宛先情報が、テーブル内で見つかった){             If (宛先情報が受信ポートに関連付けされている）                 フレームを削除 //フレームが到着したポートが宛先になっていたら送る必要なし             Else                 適切なポートに転送する         }         Else //(宛先情報が、テーブル内で見つからなかった場合)             フラッディング(受信ポート以外のすべてのポートにフレームを転送)    }
Copyright © 2024 by INIAD
40
フラッディング(Flooding): Signalを受信したポートを除いた全てのポートにそのSignalをブロードキャストすること。


（ フラッディングのイメージ）
要は
送信元がXのフレームが，ポート(インタフェース)Ａから届いたということは
ポートＡの先にXがいることを意味する
なので，それを覚えておいて，宛先がＸのフレームがきたらポートＡへ転送すればよい
Port #A
X

----------------
MAC Address Filtering  
Self Learning (自己学習)とフォワーディング (転送) を使用
スイッチが受信した各フレームに対して、 送信元MACアドレスと、それが到着したポート(受信ポート)を学習テーブルのエントリに記入し、以降のフレームを転送するためにその知識を使用する
フレームを既知のポートに転送するか、不明な場合はフラッディングする(受信ポート以外のすべてのポートにフレームを転送) 
Pseudocode:
Initial state（初期状態）：学習テーブルは空  For (フレームが到着するごとに) {     If　(送信元情報が、テーブル内で見つかった)          テーブル内のフレーム到着の時刻を更新する     Else //(送信元情報が、テーブル内で見つからなかった場合){         テーブル内に以下を登録する:              ‐送信元MACアドレス              ‐フレーム到着のポート番号              ‐フレーム到着の時刻     }
        If（宛先情報が、テーブル内で見つかった){             If (宛先情報が受信ポートに関連付けされている）                 フレームを削除 //フレームが到着したポートが宛先になっていたら送る必要なし             Else                 適切なポートに転送する         }         Else //(宛先情報が、テーブル内で見つからなかった場合)             フラッディング(受信ポート以外のすべてのポートにフレームを転送)    }
Copyright © 2024 by INIAD
40
フラッディング(Flooding): Signalを受信したポートを除いた全てのポートにそのSignalをブロードキャストすること。


（ フラッディングのイメージ）
要は
送信元がXのフレームが，ポート(インタフェース)Ａから届いたということは
ポートＡの先にXがいることを意味する
なので，それを覚えておいて，宛先がＸのフレームがきたらポートＡへ転送すればよい
Port #A
X

----------------
MAC Address Filtering  
Self Learning (自己学習)とフォワーディング (転送) を使用
スイッチが受信した各フレームに対して、 送信元MACアドレスと、それが到着したポート(受信ポート)を学習テーブルのエントリに記入し、以降のフレームを転送するためにその知識を使用する
フレームを既知のポートに転送するか、不明な場合はフラッディングする(受信ポート以外のすべてのポートにフレームを転送) 
Pseudocode:
Initial state（初期状態）：学習テーブルは空  For (フレームが到着するごとに) {     If　(送信元情報が、テーブル内で見つかった)          テーブル内のフレーム到着の時刻を更新する     Else //(送信元情報が、テーブル内で見つからなかった場合){         テーブル内に以下を登録する:              ‐送信元MACアドレス              ‐フレーム到着のポート番号              ‐フレーム到着の時刻     }
        If（宛先情報が、テーブル内で見つかった){             If (宛先情報が受信ポートに関連付けされている）                 フレームを削除 //フレームが到着したポートが宛先になっていたら送る必要なし             Else                 適切なポートに転送する         }         Else //(宛先情報が、テーブル内で見つからなかった場合)             フラッディング(受信ポート以外のすべてのポートにフレームを転送)    }
Copyright © 2024 by INIAD
40
フラッディング(Flooding): Signalを受信したポートを除いた全てのポートにそのSignalをブロードキャストすること。


（ フラッディングのイメージ）
要は
送信元がXのフレームが，ポート(インタフェース)Ａから届いたということは
ポートＡの先にXがいることを意味する
なので，それを覚えておいて，宛先がＸのフレームがきたらポートＡへ転送すればよい
Port #A
X

----------------
MAC Address Filtering  
Self Learning(自己学習)とフォワーディング (転送)の例
ステップ  0. Switch Xのテーブルは最初は空です 
イーサネットフレームの形式は次のとおりです 
Copyright © 2024 by INIAD
41
［Dst Address］ | ［Src Address]｜［Type] ｜［Payload］｜［FCS］
Port #1
Port #2
Port #3
Port #4
Learning Table for Switch X (Initially empty)
Switch X
Port #

MAC Address

Time
























*簡単のため，宛先アドレスと送信元アドレスフィールドのみが示されます 

----------------
MAC Address Filtering  
自己学習とフォワーディングの例
Step 1. スイッチXは時刻t1にポート＃1から次のフレームを受信すると，それに応じてテーブルを更新し，そのフレームをフラッディングする 
Copyright © 2024 by INIAD
42
Port #1
Port #2
Port #3
Port #4
Learning Table for Switch X
Switch X
Port #

MAC Address

Time

1

00-11-22-33-33-33

t1


















［00-55-66-77-88-99］ | ［00-11-22-33-33-33]｜［xxxx] ｜［xxxx］｜［xxxx］

----------------
MAC Address Filtering  
自己学習とフォワーディングの例
Step 1. スイッチXは時刻t1にポート＃1から次のフレームを受信すると，それに応じてテーブルを更新し，そのフレームをフラッディングする 

Step 2. スイッチXは時刻t2にポート＃3から次のフレームを受信すると， それに応じてテーブルを更新し，そのフレームをフラッディングする
Copyright © 2024 by INIAD
43
［00-99-88-77-66-55］ | ［ 00-11-33-00-cc-bb]｜［xxxx] ｜［xxxx］｜［xxxx］
［00-55-66-77-88-99］ | ［00-11-22-33-33-33]｜［xxxx] ｜［xxxx］｜［xxxx］
Port #

MAC Address

Time

1

00-11-22-33-33-33

t1

3

00-11-33-00-cc-bb

t2












Port #1
Port #2
Port #3
Port #4
Learning Table for Switch X
Switch X

----------------
MAC Address Filtering  
自己学習とフォワーディングの例
Step 3. スイッチXは時刻t3にポート＃4から次のフレームを受信すると，それに応じてテーブルを更新し，ポート＃3を介してフレームを転送する 




Copyright © 2024 by INIAD
44
［ 00-11-33-00-cc-bb] |［00-11-22-aa-bb-cc]｜［xxxx］｜［xxxx］｜［xxxx］

Port #

MAC Address

Time

1

00-11-22-33-33-33

t1

3

00-11-33-00-cc-bb

t2

4

00-11-22-aa-bb-cc

t3






Port #1
Port #2
Port #3
Port #4
Learning Table for Switch X
Switch X

----------------
MAC Address Filtering  
自己学習とフォワーディングの例
Step 3. スイッチXは時刻t3にポート＃4から次のフレームを受信すると，それに応じてテーブルを更新し，ポート＃3を介してフレームを転送する 


Step 4. スイッチXは時刻t4にポート＃4から次のフレームを受信すると，それに応じてテーブルを更新し，ポート#1を介してフレームを転送する
Copyright © 2024 by INIAD
45
［00-11-22-33-33-33] |［ 00-11-22-aa-bb-cc]｜［xxxx］｜［xxxx］｜［xxxx］

［ 00-11-33-00-cc-bb] |［00-11-22-aa-bb-cc]｜［xxxx］｜［xxxx］｜［xxxx］

Port #

MAC Address

Time

1

00-11-22-33-33-33

t1

3

00-11-33-00-cc-bb

t2

4

00-11-22-aa-bb-cc

t4






Port #1
Port #2
Port #3
Port #4
Learning Table for Switch X
Switch X

----------------
5. ARP - Address Resolution Protocol
Copyright © 2024 by INIAD
46

----------------
なぜIPアドレスとMACアドレスが必要？
Copyright © 2024 by INIAD
47
LANは任意のネットワーク層プロトコル用に設計されているため、MACアドレスではなくIPアドレスが割り当てられていると、アダプターは他のネットワーク層プロトコルをサポートできません。
アダプターにMACアドレスではなくIPアドレスを使用した場合は、ネットワーク層アドレスをアダプターのRAMに格納し、アダプターを移動または起動するたびに再設定する必要があります。
アドレスがアダプターで使用されていない場合、各フレームのデータはネットワークレイヤスタックに渡されますが、となるとホストはLAN上で送信されているすべてのフレームによって中断されてしまうことになります


----------------
アドレス変換の必要性
Copyright © 2024 by INIAD
48
ネットワーク層アドレス(IPアドレス)とデータリンク層アドレス(MACアドレス)の両方があるので、それらの間でアドレスを変換する必要がある。
IPアドレスが分かれば，宛先IPアドレスに向けてパケットを送信することができます．
しかし，実際にデータリンクを利用して通信するときにはIPアドレスに対応したMACアドレスが必要になります．

----------------
アドレス変換の必要性
Copyright © 2024 by INIAD
48
ネットワーク層アドレス(IPアドレス)とデータリンク層アドレス(MACアドレス)の両方があるので、それらの間でアドレスを変換する必要がある。
IPアドレスが分かれば，宛先IPアドレスに向けてパケットを送信することができます．
しかし，実際にデータリンクを利用して通信するときにはIPアドレスに対応したMACアドレスが必要になります．

----------------
アドレス変換の必要性
Copyright © 2024 by INIAD
48
ネットワーク層アドレス(IPアドレス)とデータリンク層アドレス(MACアドレス)の両方があるので、それらの間でアドレスを変換する必要がある。
IPアドレスが分かれば，宛先IPアドレスに向けてパケットを送信することができます．
しかし，実際にデータリンクを利用して通信するときにはIPアドレスに対応したMACアドレスが必要になります．

----------------
IPアドレスとMACアドレス間のマッピング
Copyright © 2024 by INIAD
49
受信者のIPアドレスが特定された後でも、 受信者（あるいは中継者＝ルータ）のMACアドレスも知る必要があります
各デバイスはネットワークインタフェースカード（NIC）を持っていて，MACアドレスを持っています
各デバイスは、受信者（あるいは中継者）のMACアドレスを見つける必要があります

----------------
Copyright © 2024 by INIAD
特定のネットワーク層アドレス（IPv4アドレスなど）に関連付けられたデータリンク層アドレス（MACアドレスなど）をみつけるために使用される通信プロトコル
通信相手のIPアドレスがわかっているデバイスの場合は、「あなたのMACアドレスは何ですか？」という質問を送信するだけで、未知のMACアドレスを見つけることができます。 
ARPリクエストをブロードキャストする
該当するデバイスがARPレスポンスメッセージで返答する
取得したMACアドレスはARPテーブルに保持されます
IPアドレスとMACアドレスのペアのリストで構成されています
テーブルのコンテンツは、以降関連するデバイスにコンタクトするときに使用される
ARP (Address Resolution Protocol)
50

----------------
Copyright © 2024 by INIAD
IP address: 192.168.3.6
IP address: 192.168.3.3
MAC address: DD-33-23-22-22-BC
IP address: 192.168.3.5
IP address: 192.168.3.4
IP address: 192.168.3.10
MAC address: CC-2F-71-46-A6-EA
ARP Request:
SrcIPaddr: 192.168.3.10
DstIPaddr: 192.168.3.3
SrcMACaddr: CC-2F-71-46-A6-EA
DstMACaddr: FF-FF-FF-FF-FF-FF (broadcast)
(“Mrs. 192.168.3.3, please tell me your MAC address! I am CC-2F-71-46-A6-EA at 192.168.3.10”)

ARP Response: 
SrcIPaddr: 192.168.3.3
DstIPaddr: 192.168.3.10
SrcMACaddr: DD-33-23-22-22-BC DstMACaddr: CC-2F-71-46-A6-EA
(“Mr. CC-2F-71-46-A6-EA at 192.168.3.10, I am 192.168.3.3, and my MAC address is DD-33-23-22-22-BC!)

ARP (Address Resolution Protocol)
通信相手のIPアドレスがわかっているデバイスの場合は、単に「あなたのMACアドレスは何ですか？」と尋ねて、そのMACアドレスを調べます。
ブロードキャストARPリクエストを介して行われます。
51

----------------
Copyright © 2024 by INIAD
IP address: 192.168.3.6
IP address: 192.168.3.3
MAC address: DD-33-23-22-22-BC
IP address: 192.168.3.5
IP address: 192.168.3.4
IP address: 192.168.3.10
MAC address: CC-2F-71-46-A6-EA
ARP Request:
SrcIPaddr: 192.168.3.10
DstIPaddr: 192.168.3.3
SrcMACaddr: CC-2F-71-46-A6-EA
DstMACaddr: FF-FF-FF-FF-FF-FF (broadcast)
(“Mrs. 192.168.3.3, please tell me your MAC address! I am CC-2F-71-46-A6-EA at 192.168.3.10”)

ARP Response: 
SrcIPaddr: 192.168.3.3
DstIPaddr: 192.168.3.10
SrcMACaddr: DD-33-23-22-22-BC DstMACaddr: CC-2F-71-46-A6-EA
(“Mr. CC-2F-71-46-A6-EA at 192.168.3.10, I am 192.168.3.3, and my MAC address is DD-33-23-22-22-BC!)

ARP (Address Resolution Protocol)
通信相手のIPアドレスがわかっているデバイスの場合は、単に「あなたのMACアドレスは何ですか？」と尋ねて、そのMACアドレスを調べます。
ブロードキャストARPリクエストを介して行われます。
51

----------------
Copyright © 2024 by INIAD
IP address: 192.168.3.6
IP address: 192.168.3.3
MAC address: DD-33-23-22-22-BC
IP address: 192.168.3.5
IP address: 192.168.3.4
IP address: 192.168.3.10
MAC address: CC-2F-71-46-A6-EA
ARP Request:
SrcIPaddr: 192.168.3.10
DstIPaddr: 192.168.3.3
SrcMACaddr: CC-2F-71-46-A6-EA
DstMACaddr: FF-FF-FF-FF-FF-FF (broadcast)
(“Mrs. 192.168.3.3, please tell me your MAC address! I am CC-2F-71-46-A6-EA at 192.168.3.10”)

ARP Response: 
SrcIPaddr: 192.168.3.3
DstIPaddr: 192.168.3.10
SrcMACaddr: DD-33-23-22-22-BC DstMACaddr: CC-2F-71-46-A6-EA
(“Mr. CC-2F-71-46-A6-EA at 192.168.3.10, I am 192.168.3.3, and my MAC address is DD-33-23-22-22-BC!)

ARP (Address Resolution Protocol)
通信相手のIPアドレスがわかっているデバイスの場合は、単に「あなたのMACアドレスは何ですか？」と尋ねて、そのMACアドレスを調べます。
ブロードキャストARPリクエストを介して行われます。
51

----------------
Copyright © 2024 by INIAD
IP address: 192.168.3.6
IP address: 192.168.3.3
MAC address: DD-33-23-22-22-BC
IP address: 192.168.3.5
IP address: 192.168.3.4
IP address: 192.168.3.10
MAC address: CC-2F-71-46-A6-EA
ARP Request:
SrcIPaddr: 192.168.3.10
DstIPaddr: 192.168.3.3
SrcMACaddr: CC-2F-71-46-A6-EA
DstMACaddr: FF-FF-FF-FF-FF-FF (broadcast)
(“Mrs. 192.168.3.3, please tell me your MAC address! I am CC-2F-71-46-A6-EA at 192.168.3.10”)

ARP Response: 
SrcIPaddr: 192.168.3.3
DstIPaddr: 192.168.3.10
SrcMACaddr: DD-33-23-22-22-BC DstMACaddr: CC-2F-71-46-A6-EA
(“Mr. CC-2F-71-46-A6-EA at 192.168.3.10, I am 192.168.3.3, and my MAC address is DD-33-23-22-22-BC!)

ARP (Address Resolution Protocol)
通信相手のIPアドレスがわかっているデバイスの場合は、単に「あなたのMACアドレスは何ですか？」と尋ねて、そのMACアドレスを調べます。
ブロードキャストARPリクエストを介して行われます。
51

----------------
Copyright © 2024 by INIAD
52
WiresharkによってキャプチャされたARPパケットの中身を確認してみる
Hardware type: イーサネットの場合は1
Protocol type: 上位層プロトコルタイプ（IPv4の場合は0800）
HLEN: ハードウェアアドレスサイズ（MACアドレスの場合は6）
PLEN: 上位層プロトコルアドレスサイズ（IPアドレスの場合は4）
Opcode: ARP要求の場合は1、ARP応答の場合は2
Sender MAC address: 送信元MACアドレス
Sender IP address: 送信元IPアドレス
Target MAC address:ターゲットMACアドレス
Target IP address:ターゲットIPアドレス
［Ethernet Header］ | ［ARP message]      ｜［FCS］
ARP メッセージフォーマット 

----------------
Copyright © 2024 by INIAD
53
例)
IP address: 192.168.3.6
MAC address: D8-0F-99-32-7F-F3

IP address: 192.168.3.3
MAC address: DD-33-23-22-22-BC


IP address: 192.168.3.5
IP address: 192.168.3.4
IP address: 192.168.3.10
MAC address: CC-2F-71-46-A6-EA
ARP Request:
SrcIPaddr: 192.168.3.6
DstIPaddr: 192.168.3.10
SrcMACaddr: D8-0F-99-32-7F-F3
DstMACaddr: FF-FF-FF-FF-FF-FF (broadcast)
(“Mr. 192.168.3.10, please tell me your MAC address! I am D8-0F-99-32-7F-F3
 at 192.168.3.6”)

ARP リクエストメッセージ

----------------
Copyright © 2024 by INIAD
53
例)
IP address: 192.168.3.6
MAC address: D8-0F-99-32-7F-F3

IP address: 192.168.3.3
MAC address: DD-33-23-22-22-BC


IP address: 192.168.3.5
IP address: 192.168.3.4
IP address: 192.168.3.10
MAC address: CC-2F-71-46-A6-EA
ARP Request:
SrcIPaddr: 192.168.3.6
DstIPaddr: 192.168.3.10
SrcMACaddr: D8-0F-99-32-7F-F3
DstMACaddr: FF-FF-FF-FF-FF-FF (broadcast)
(“Mr. 192.168.3.10, please tell me your MAC address! I am D8-0F-99-32-7F-F3
 at 192.168.3.6”)

ARP リクエストメッセージ

----------------
Copyright © 2024 by INIAD
53
例)
IP address: 192.168.3.6
MAC address: D8-0F-99-32-7F-F3

IP address: 192.168.3.3
MAC address: DD-33-23-22-22-BC


IP address: 192.168.3.5
IP address: 192.168.3.4
IP address: 192.168.3.10
MAC address: CC-2F-71-46-A6-EA
ARP Request:
SrcIPaddr: 192.168.3.6
DstIPaddr: 192.168.3.10
SrcMACaddr: D8-0F-99-32-7F-F3
DstMACaddr: FF-FF-FF-FF-FF-FF (broadcast)
(“Mr. 192.168.3.10, please tell me your MAC address! I am D8-0F-99-32-7F-F3
 at 192.168.3.6”)

ARP リクエストメッセージ

----------------
Copyright © 2024 by INIAD
53
例)
IP address: 192.168.3.6
MAC address: D8-0F-99-32-7F-F3

IP address: 192.168.3.3
MAC address: DD-33-23-22-22-BC


IP address: 192.168.3.5
IP address: 192.168.3.4
IP address: 192.168.3.10
MAC address: CC-2F-71-46-A6-EA
ARP Request:
SrcIPaddr: 192.168.3.6
DstIPaddr: 192.168.3.10
SrcMACaddr: D8-0F-99-32-7F-F3
DstMACaddr: FF-FF-FF-FF-FF-FF (broadcast)
(“Mr. 192.168.3.10, please tell me your MAC address! I am D8-0F-99-32-7F-F3
 at 192.168.3.6”)

ARP リクエストメッセージ

----------------
Copyright © 2024 by INIAD
54
ARP レスポンスメッセージ
例)
IP address: 192.168.3.6
MAC address: D8-0F-99-32-7F-F3

IP address: 192.168.3.3
MAC address: DD-33-23-22-22-BC


IP address: 192.168.3.5
IP address: 192.168.3.4
IP address: 192.168.3.10
MAC address: CC-2F-71-46-A6-EA
ARP Response (reply): 
SrcIPaddr: 192.168.3.10
DstIPaddr: 192.168.3.6
SrcMACaddr: CC-2F-71-46-A6-EA DstMACaddr: D8-0F-99-32-7F-F3
(“Mr. D8-0F-99-32-7F-F3 at 192.168.3.6, I am 192.168.3.10, and my MAC address is CC-2F-71-46-A6-EA!)


----------------
Copyright © 2024 by INIAD
54
ARP レスポンスメッセージ
例)
IP address: 192.168.3.6
MAC address: D8-0F-99-32-7F-F3

IP address: 192.168.3.3
MAC address: DD-33-23-22-22-BC


IP address: 192.168.3.5
IP address: 192.168.3.4
IP address: 192.168.3.10
MAC address: CC-2F-71-46-A6-EA
ARP Response (reply): 
SrcIPaddr: 192.168.3.10
DstIPaddr: 192.168.3.6
SrcMACaddr: CC-2F-71-46-A6-EA DstMACaddr: D8-0F-99-32-7F-F3
(“Mr. D8-0F-99-32-7F-F3 at 192.168.3.6, I am 192.168.3.10, and my MAC address is CC-2F-71-46-A6-EA!)


----------------
Copyright © 2024 by INIAD
54
ARP レスポンスメッセージ
例)
IP address: 192.168.3.6
MAC address: D8-0F-99-32-7F-F3

IP address: 192.168.3.3
MAC address: DD-33-23-22-22-BC


IP address: 192.168.3.5
IP address: 192.168.3.4
IP address: 192.168.3.10
MAC address: CC-2F-71-46-A6-EA
ARP Response (reply): 
SrcIPaddr: 192.168.3.10
DstIPaddr: 192.168.3.6
SrcMACaddr: CC-2F-71-46-A6-EA DstMACaddr: D8-0F-99-32-7F-F3
(“Mr. D8-0F-99-32-7F-F3 at 192.168.3.6, I am 192.168.3.10, and my MAC address is CC-2F-71-46-A6-EA!)


----------------
Copyright © 2024 by INIAD
54
ARP レスポンスメッセージ
例)
IP address: 192.168.3.6
MAC address: D8-0F-99-32-7F-F3

IP address: 192.168.3.3
MAC address: DD-33-23-22-22-BC


IP address: 192.168.3.5
IP address: 192.168.3.4
IP address: 192.168.3.10
MAC address: CC-2F-71-46-A6-EA
ARP Response (reply): 
SrcIPaddr: 192.168.3.10
DstIPaddr: 192.168.3.6
SrcMACaddr: CC-2F-71-46-A6-EA DstMACaddr: D8-0F-99-32-7F-F3
(“Mr. D8-0F-99-32-7F-F3 at 192.168.3.6, I am 192.168.3.10, and my MAC address is CC-2F-71-46-A6-EA!)


----------------
Copyright © 2024 by INIAD
55
216.58.196.22
IP address: 192.168.3.10
MAC address: CC-2F-71-46-A6-EA
SrcIPaddr: 192.168.3.10
DstIPaddr: 216.58.196.22
SrcMACaddr: CC-2F-71-46-A6-EA
DstMACaddr: MAC address of Router X’s Port #1

SrcIPaddr: 192.168.3.10
DstIPaddr: 216.58.196.22
SrcMACaddr: MAC address of Router X’s Port #3
DstMACaddr: : MAC address of Router Y’s Port #1
SrcIPaddr: 192.168.3.10
DstIPaddr: 216.58.196.22
SrcMACaddr: MAC address of Router Y’s Port #3
DstMACaddr: MAC address of 216.58.196.22

Router X
Router Y
宛先が同じネットワーク内にない場合は？基本的にはインターネット上の中継デバイス（ルータ）を経由するたびにMACアドレスフィールドがその中間デバイスのものに置き換えられます
1. 中継ルータのMACアドレスをブロードキャストARPリクエストで取得
2. 中継ルータのMACアドレスを宛先MACアドレスとしてフレームを送信
3. フレームを受信した中継ルータが上記のステップ１と２を繰り返す


ネットワーク外への転送時
Port #3
Port #1
Port #3
Port #1
