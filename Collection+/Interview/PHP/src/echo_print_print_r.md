# echo(),print(),print_r()的区别是什么？ #

## 答案 ##

### [echo](http://php.net/manual/zh/function.echo.php) ###
`echo` 输出一个或多个字符串  

#### 格式 ####
```php\
void echo (string $arg1 [, $string $...] )
```  
echo 不是一个函数（它是一个语言结构）， 因此你不一定要使用小括号来指明参数，单引号，双引号都可以。 echo （不像其他语言构造）不表现得像一个函数， 所以不能总是使用一个函数的上下文。 另外，如果你想给echo 传递多个参数， 那么就不能使用小括号。  
**echo 没有返回值**

------

### [print](http://php.net/manual/zh/function.print.php) ###
`print` 输出字符串

#### 格式 ####
```php
int print ( string $arg )
```  
`print`实际上不是一个函数（而是语言结构），所以尅不用圆括号包围参数列表。  
和`echo`的唯一区别: `print`仅支持一个参数  
`print`总是返回1。

------

### [print_r](http://php.net/manual/zh/function.print-r.php) ###
`print_r` 打印关于变量的易于理解的信息

#### 格式 ####
```php
bool print_r ( mixed $expression [, bool $return])
```  
`print_r()`显示关于一个变量易于理解的信息。如果给的是`string`、`integer`或`float`，将打印变量值本身。如果个的是`array`或`object`，将会按照一定的格式显示键和值。  
1`print_r()`会把数组的指针移到最后面，使用`reset`可以让指针回到开始处。
如果第二个参数设置为`TRUE`，`print_r()`将不打印结果，而是返回其输出。
