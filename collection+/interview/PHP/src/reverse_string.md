# 如何实现字符串翻转 #
PHP没有实现字符串翻转的函数，下面是一个简单的实现:

```php

$str = 'abcdefghijklmn';

$len  = strlen($str);
$half = floor($len / 2);

for ( $i = 0; $i <= $half; $i ++ ) {
    $tmp = $str[i];
    $str[$i] = $str[$len - 1 - $i];
    $str[$len - 1 - $i] = $tmp;
}

echo $str;    // 输出 nmlkjihgfedcba

```
