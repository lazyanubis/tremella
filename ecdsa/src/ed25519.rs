// https://zhuanlan.zhihu.com/p/579276183
// https://zhuanlan.zhihu.com/p/524180490
// https://cloud.tencent.com/developer/article/2437665
// 爱德华兹曲线 x^2 + y^2 = 1 + d * x^2 * y^ 2
// 扭曲爱德华兹曲线 a * x^2 + y^2 = 1 + d * x^2 * y^ 2 (a,d != 0, a !=d)
// 蒙哥马利曲线 B * y^2 = x^3 + A * x^2 + x
// Curve25519 y^2 = x^3 + 486662 * x^2 + x (p = 2^255 - 19, 基点(9, 14781619447589544791020593568409986887264606134616475288964881837755586237401))
// Ed25519 -x^2 + y^2 = 1 - (121665/121666) * x^2 * y^2
// 基点 0x216936d3cd6e53fec0a4e231fdd6dc5c692cc7609525a7b2c9562d608f25d51a,0x6666666666666666666666666666666666666666666666666666666666666658
// 阶 2^252 + 27742317777372353535851937790883648493

#[test]
fn test() {
    // const p =
}