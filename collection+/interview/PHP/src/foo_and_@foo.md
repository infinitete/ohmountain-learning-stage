# 简述foo()和@foo()的区别 #

## 答案 ##
PHP的@运算符只对表达式有效，而不能用在语句中，意思是忽略此处的警告(而不是错误)。如:  
``` php
<?php
echo $null_variable;    // PHP Notice: Undefined variable
echo @$null_variable;   // 不会有任何提示(NOTICE)

// 然而
echo @function;         // PHP Parse error:  syntax error, unexpected ';', expecting '('，这是个ERROR，@无法抑制ERROR
```
