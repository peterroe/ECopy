---
title: 巧妙的位运算
date: 2021-11-27
time: 5min
---

>位运算在处理数字间的关系中总是表现出色

基本的与或非这里就不在多说了，来看看下面几个巧妙的位运算例子：

## 获得目标比特位

```javascript
function getBit(number, bitPosition) {
    return (number >> bitPosition) & 1
}
```
该方法向右移动目标位到最右边，即位数组的第**0**个位置上。然后在该数上与形如**0001**的二进制形式的数进行**ADD**操作。这会清理掉除了目标位的所有其它位的数据。如果目标位是**1**，那么结果就是**1**，反之，结果是**0**

**解释**

假设我们要获得数字**23**的第三个比特位，那么就是向右边移动**2**位

所以**number=23、bitPosition=2**

```js
//所以:
23 >> 2 & 1
//等价于:
(10111)₂ >> 2 & 1
//等价于为:
(101)₂ & 1
//位运算结果:
1
```
## 把目标位设置为1

```js
function setBit(number,bitPosition) {
    return (1 << bitPosition) && number
}
```
该方法把**1**向左移动了`bitPosition`位，生成了一个二进制形如**00100**的值。然后我们拿该值与目标数字进行`OR`操作，就能把目标位设置位**1**而不影响其它位

**解释**

假设**number=23、bitPosition=3**

```js
1 << 3 & 23
//等价于:
(1000)₂ << 3 & (10111)₂
//等价于为:
(1000)₂ & (10111)₂
//位运算结果:
(11111)₂
//十进制：
31
```

## 把目标位设置为0

```js
function clearBit() {
    return ~(1 << bitPosition) & number
}
```
该方法把**1**向左移动了`bitPosition`位，生成了一个二进制形如**00100**的值。然后反转每一位的数字，得到一个二进制形如**11011**的值。接着与目标值进行`ADD`操作，就能清除掉目标位的值

**~取反操作**

`~`操作会将二进制位逐位取反，包括符号位

## 把目标为设置为0或1

```js
function updateBit(number,bitPosition,bitValue) {
    (bitValue <<< bitPosition) |  //设置目标值
    (number & ~(1 << bitPosition)) //清除目标值
}
```

上面方法组合了`ClearBit`和`SetBit`

**为什么要清除目标值？直接设置不行吗？**

答案是不行的，因为我们不知道目标位（假设为`X`）是**1**还是**0**，直接计算就会有不确定的结果，例如下:
```js
X | 0 = 0 //(X = 0)
X | 0 = 1 //(X = 1)
X | 1 = 0 //(X = 1)
X | 1 = 1 //(X = 1)
```
而我们并不关心`X`原来是**0**还是**1**，所以先把`X`值置为零，就可以得到绝对正确的结果
```js
X | 0 = 0 //(X = 0)
X | 1 = 1 //(X = 0)
```

## 检测某一个数奇偶性
```js
function isOdd(number) {
    return number & 1
}
```
奇数的最后一位一定为1

## 数字本身乘2
```js
function mutiplyByTwo(number) {
    return number << 1
}
```

该方法将原始数字向左移动一位。因此所有位都将乘以**2**，因此数字本身也将乘以**2**

## 数字本身除2
```js
function divideByTwo(number) {
    return number >> 1
}
```

与上同理，向右移动即除2

## 改变数字符号
```js
function switchSign(number) {
  return ~number + 1;
}
```
该方法将正数变成负数，反之亦然。为了做到这一点，它使用了**二进制补码**的方法，即取反所有位然后加1

## 计算数字的有效位数
```js
function bitLength(number) {
    let bitsCounter = 0;

    while ((1 << bitsCounter) <= number) {
        bitsCounter += 1;
    }

    return bitsCounter;
}
```
例如`number = 5`，也就是`(0101)₂`，有三个有效位

模拟步骤:
* 1 小于或等于 5，计数为1
* 2 小于或等于 5，计数为2
* 4 小于或等于 5，计数为3
* 8 大于5，停止计数

## 判断数字是否可以表示为2的幂
```js
function isPowerOfTwo(number) {
  return (number & (number - 1)) === 0;
}
```
先来思考下，常见2的幂有：2、4、8、16

以16为例子：

`16 & 15` 等价于`(1000)₂ & (0111)₂`，运算之后结果自然为**0**

## 位运算有什么用？

虽然位运算通过各种左移右移来操作数字的过程甚是奇妙，但可能你也会思考，这些操作我不用位运算也能做到啊？那么为什么我们还需要位运算呢？

其实不然，位运算的作用在算法中经常体现

例如`LeetCode`的[这一个题目](https://leetcode-cn.com/problems/single-number/)：


![image.png](https://p1-juejin.byteimg.com/tos-cn-i-k3u1fbpfcp/d98aee61d84c45059c46291050fdb514~tplv-k3u1fbpfcp-watermark.image?)

显然，因为相同的数字**异或一定为0**，剩下的那个数字肯定会保留下来，根据异或的交换性，我们不用管数字的顺序，异或每一个数字即可：

```js
var singleNumber = function(nums) {
    let s = 0
    for(let num of nums) {
        s ^= num
    }
    return s
};
```

可见，掌握位运算还有必要的😁