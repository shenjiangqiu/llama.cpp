## ggml quantization

### 1. float to Q8: GGML: ggml-quants.c: 2330
1. 一个block 有256 个元素，对于每一个block，找到绝对值最大的值 amax = 1.5，max=-1.5，计算得到 iscale = -128/max
2. 对于每一个元素，得到 v = nearest_int(iscale * x[j])
3. 在一个block（256）里面，每16个分为一组，得到一个sum，这样就有16个sum
4. 最有保存到一个block，每个block 有3个值，d = 1/iscale , bsum = 16 个sum， qs 是256个元素的量化值

### 2. Q2 * Q8: ggml-quants.c:3648
1. 对于每个block 计算一个sumf, 最后累加所有的sumf
2. sumf += dall * isum - dmin*summs.
3. summs = 

```c++
const uint8_t * sc = x[i].scales;
for (int j = 0; j < 16; ++j) {
    summs += y[i].bsums[j] * (sc[j] >> 4);
}
```

4. `dall = y[i].d * x[i].d` , `dmin = y[i].d * x[i].dmin`
5. q8 的遍历顺序如常，q2的便利顺序每32个为一组，每一组对应q8的32个数，然后像右边移动2个比特。持续4次一共8个比特。
6. 所以q2 每走32byte， q8正好走32*4 byte
8. 每一次计算是 q8 * q2 * d. 其中d是q2 的scales（16个一组）
9. 最后得到的计算结果是isum

### data structure:
```c++
typedef struct {
    uint8_t scales[QK_K/16]; // scales and mins, quantized with 4 bits
    uint8_t qs[QK_K/4];      // quants
    half d;           // super-block scale for quantized scales
    half dmin;        // super-block scale for quantized mins
} block_q2_K;


typedef struct {
    float   d;              // delta
    int8_t  qs[QK_K];       // quants
    int16_t bsums[QK_K/16]; // sum of quants in groups of 16
} block_q8_K;
```

### conclusion:
- q2:  d2*qs2[]*scales2[] - dmin2
- q8: d8*qs8[]
- q2*q8 = d2*d8*qs2[]*scals2[]*qs8[] - dmin2*d8*qs8[]
- 所以在第二个部分 dmin2 * d8 就是 dmin，然后qs8就是 summs
- 在第一个部分，循环部分计算了qs2[]*scals2[]*qs8[]， 最后d2*d8就是dall