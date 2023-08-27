# 『アニメーション』ノート

（最終更新： 2023-08-26）


## 目次

1. [transition](#transition)
1. [animation](#animation)
	1. [keyframes](#keyframes)


## transform

**transform**は、[CSS](./css.md#css)において、[要素](../../../html/_/chapters/html.md#要素)を回転、拡大縮小、傾斜、移動するための[プロパティ](./css.md#プロパティ)。 `translate` （移動）、 `scale` （拡大縮小）、 `rotate` （回転）、 `skew` （傾斜）などを適用することができる。


## transition

**transition**は、[CSS](./css.md#css)において、[プロパティ](./css.md#プロパティ)の変更を検知し、その変化の仕方を制御するための[プロパティ](./css.md#プロパティ)。変化にかかる時間、変化を検知する対象の[プロパティ](./css.md#プロパティ)、変化の仕方、変化が始まるまでの遅延を一度に適用することができる[ショートハンドプロパティ](./css.md#ショートハンドプロパティ)。

各スタイルを個別に指定する[プロパティ](./css.md#プロパティ)も存在し、変化にかかる時間を指定する**transition-duration**、変化を検知する対象の[プロパティ](./css.md#プロパティ)を指定する**transition-property**、変化の仕方を指定する**transition-timing-function**、変化が始まるまでの遅延を指定する**transition-delay**が使用できる。 `transition-timing-function` には、 `linear` （等速で変化）、 `ease-in` （徐々に加速）、 `ease-out` （徐々に減速）、 `ease-in-out` （加速してから減速）といったキーワードが指定できる。


## animation

**animation**は、[CSS](./css.md#css)において、アニメーションを適用するための[プロパティ](./css.md#プロパティ)。 `keyframes` アニメーション名を指定して、繰り返し回数、再生方向、開始前と終了後の状態、実行状態を一度に適用できる[ショートハンドプロパティ](./css.md#ショートハンドプロパティ)。

各スタイルを個別に設定する[プロパティ](./css.md#プロパティ)も存在し、アニメーション名を指定する**animation-name**、繰り返し回数を指定する**animation-iteration-count**（無限に実行したい場合は `infinite` を指定）、再生方向を指定する**animation-direction**、開始前と終了時の状態を指定する**animation-fill-mode**、実行状態を指定する**animation-play-state**が使用できる。 `animation-direction` には、 `normal` （毎回順方向）、 `reverse` （毎回逆方向）、 `alternate` （順方向から始まり、交互）、 `alternate-reverse` （逆方向から始まり、交互）と行ったキーワードが指定できる。 `animation-fill-mode` には、 `none` （適用しない）、 `forwards` （終了後は終了時のスタイルを適用）、 `backwards` （開始前に開始時のスタイルを適用）、 `both` （ `forwards` と `backwards` の両方を適用）といったキーワードが指定できる。 `animation-play-state` には、 `running` か `paused` のどちらかが指定できる。


### keyframes

**keyframes**は、[CSS](./css.md#css)において、アニメーションの流れに沿ったキーフレーム（中間地点）のスタイルを定義することによって、アニメーションを制御する機能。キーフレームの使用例は以下の通り。

```css
@keyframes slidein
{
    from
    {
        transform: translateX(0%);
    }

    to
    {
        transform: translateX(100%);
    }
}

@keyframes box_move
{
    0%
    {
        top: 0;
        left: 0;
    }

    25%
    {
        top: 50px;
    }

    50%
    {
        left: 50px;
    }

    75%
    {
        top: 0;
    }

    100%
    {
        left: 0;
    }
}
```
