# 打印上一月最后一天的当前时间 #

## 答案  ##

幸亏PHP有个函数叫[`strtotime`](http://php.net/manual/zh/function.strtotime.php)。  
同时需要了解[`date`](http://php.net/manual/zh/function.date.php)的格式。

``` php

// 握槽，真这么简单？直接就减一个月？
echo date('Y-m-t H:i:s', strtotime('-1 month')); 

// date 格式中的 t 表示上个月有多少天，即上个月的最后一天
```
