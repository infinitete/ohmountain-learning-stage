# 写一个遍历目录下所有文件和文件夹的函数 #

## 答案 ##

```php
 /**
 * 遍历目录
 *
 * @param string $path
 */
function traversalDirectory(string $path) {

    $path = realpath($path);

    if ($path !== false && is_file($path)) {
        echo $path . "\n";
        return;
    }

    $dir = dir($path);

    /**
     * 防止打开文件目录失败
     */
    if (!($dir instanceof Directory)) {
        return;
    }

    $handle = $dir->handle;

    while (($_path = readdir($handle)) !== false) {

        if ($_path == '.' || $_path == '..') {
            continue;
        }

        $_path = $path.'/'.$_path;

        if (is_file($_path)) {
            echo $_path . "\n";
            continue;
        }
            
        traversalDirectory($_path);
    }

    /**
     * 关闭文件句柄
     */
    $dir->close();
}

```

## 要点 ##
1. [Directory](http://php.net/zh/manual/class.directory.php)
2. [dir](http://php.net/zh/manual/function.dir.php)
3. [readdir](http://php.net/zh/manual/function.readdir.php)

## 陷阱  ##
1. 千万要过滤掉..和.这两个特殊目录，否则函数会成为死函数
2. 千万要获取文件的真实目录，因为`readdir`返回的只是一个相对文件名或者路径名
