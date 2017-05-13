# 检测一个变量是否设置的函数是什么？ #
[相关连接](http://php.net/manual/zh/function.isset.php)

## 答案  ##
检测一个变量是否设置的函数是`isset`,格式为:  
```php
bool isset( mixed $var [, mixed $...] )
```

如果`$var`存在并且值不是`NULL`,则返回`TRUE`,否则返回`FALSE`
