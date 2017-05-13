# 检测一个变量是否为空的函数是什么？ #
[相关连接](http://php.net/manual/zh/function.empty.php)

## 答案  ##
检测一个变量是否为空的函数是`empty`，格式为:  
```php
bool empty( mixed $var )
```  
当一个变量并不存在，或者她的值等同于`FALSE`, 那么他会被认为不存在。如果变量不存在，`empty()`并不会产生警告。  
当`var`存在，并且是一个非空非零的值时返回 FALSE 否则返回 TRUE。
