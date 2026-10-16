# a

在游戏模拟中，你需要在计算量可以接受的情况下给出足够精确的公式，如果有需要你可以提出额外的参数。

我期望游戏能够在任意维度$DIM$运行。

```rust
/// 计算使用的数字
pub type Num=...;
/// 维度
pub const DIM:usize=...;
```

他们实际上是泛型参数，但是我会省略他们

## 星尘

在N维空间中有大量粒子$P_s$，粒子的速度服从各向同性的正态分布，

粒子总质量$M(P_s)=\sum m_i$，总动量$Mom(P_s)=\sum m_i v_i$，总能量$E(P_s)=\sum 0.5 m_i v_i^2$，空间体积V，

假设这些粒子无论受到什么影响，在经过dt时间后都能变成服从正态分布。

```rust
pub struct Matter{
    mass:Num,
    momentum:SVector<Num,DIM>,
    energy:Num,
    volume:Num,
}
let ps:Matter=...;
```

## 光

现在一束稳定的光穿过这片粒子，波长$\lambda$，能量/s$e_lps$，宽度???，经过的长度$l_l$

求光穿过这片粒子后的状态，和经过$dt$时间大量粒子的状态的变化。

## 粒子群交互

假设有两组粒子$P_A$，$P_B$，其有体积$V_{AB}$发生交互，经过$dt$时间，给出这两组粒子的变化

```rust
/// # Return
/// matter from a to b
pub fn matter_interact(a:Matter,b:Matter,dt:Num)->Matter;
```

## 武器：高能类粒子群

设计武器参数，

## 固体

一个固体经过大量粒子

固体体积$V_o$，质量$M_o$，速度$V_o$，内能$$