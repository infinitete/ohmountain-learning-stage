# 用PHP打印出前一天的时间,格式是2017-05-10 22:21:21 #
这个功能需要使用两个函数:
`[time](http://php.net/manual/zh/function.time.php)`和`[date](http://php.net/manual/zh/function.date)`,`time`用来获取当前的时间戳，而`date`用来格式化时间。

## 答案  ##
```php

$yes = time() - (3600 * 24)  // 昨天此刻的时间戳
$fmt = 'Y-m-d H:i:s';

echo date($fmt, $yes);       // 输出 2017-05-12 14:03:31

```
