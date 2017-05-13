# MySQL存储引擎 #

## 存储引擎 ##
存储引擎其实就是如何实现存储数据，如何为存储的数据建立索引以及如何更新，查询数据等技术实现的方法。  
MySQL中的数据用各种不同的技术存储在文件（或内存）中，这些技术中的每一种技术都使用不同的存储机制，索引技巧，锁定水平并且最终提供广泛的不同功能和能力。在MySQL中将这些不同的技术及配套的相关功能称为存储引擎。

在MySQL中查看存储引擎：

``` shell
mysql> show engines;
+--------------------|---------|----------------------------------------------------------------|--------------|------|------------+
| Engine             | Support | Comment                                                        | Transactions | XA   | Savepoints |
+--------------------|---------|----------------------------------------------------------------|--------------|------|------------+
| MEMORY             | YES     | Hash based, stored in memory, useful for temporary tables      | NO           | NO   | NO         |
| MRG_MYISAM         | YES     | Collection of identical MyISAM tables                          | NO           | NO   | NO         |
| InnoDB             | DEFAULT | Supports transactions, row-level locking, and foreign keys     | YES          | YES  | YES        |
| BLACKHOLE          | YES     | /dev/null storage engine (anything you write to it disappears) | NO           | NO   | NO         |
| CSV                | YES     | CSV storage engine                                             | NO           | NO   | NO         |
| MyISAM             | YES     | MyISAM storage engine                                          | NO           | NO   | NO         |
| ARCHIVE            | YES     | Archive storage engine                                         | NO           | NO   | NO         |
| PERFORMANCE_SCHEMA | YES     | Performance Schema                                             | NO           | NO   | NO         |
| FEDERATED          | NO      | Federated MySQL storage engine                                 | NULL         | NULL | NULL       |
+--------------------|---------|----------------------------------------------------------------|--------------|------|------------+
9 rows in set (0.00 sec)  
```
可以看到，MySQL有9重存储引擎，常用的存储引擎有InnoDB、MyISAM、MEMORY、BLACKHOLE等。

### InnoDB ###
1. 通过查看上面的表格，存储引擎的mysql表提供了事务，回滚以及系统崩溃修复能力和多版本迸发控制的事务的安全。
2. InnoDB支持自增长列（auto_increment）,自增长列的值不能为空，如果在使用的时候为空的话怎会进行自动存现有的值开始增值，如果有但是比现在的还大，则就保存这个值。
3. InnoDB存储引擎支持外键（foreign key） ,外键所在的表称为子表而所依赖的表称为父表。
4. InnoDB存储引擎最重要的是支持事务，以及事务相关联功能。
5. InnoDB存储引擎支持mvcc的行级锁。
6. InnoDB存储引擎索引使用的是B+Tree。

### MyISAM ###
1. MyISAM存储引擎不支持事务，不支持行级锁，只支持并发插入的表锁，主要用于高负载的select。
2. MyISAM存储引擎支持三种不同的存储结构：静态型、动态型、压缩型。
    1. 静态类型：就是定义的表列的大小是固定（即不含有：xblob、xtext、varchar等长度可变的数据类型），这样mysql就会自动使用静态myisam格式。使用静态格式的表的性能比较高，因为在维护和访问的时候以预定格式存储数据时需要的开销很低。但是这高性能是有空间换来的，因为在定义的时候是固定的，所以不管列中的值有多大，都会以最大值为准，占据了整个空间。
    2. 动态类型：如果列（即使只有一列）定义为动态的（xblob, xtext, varchar等数据类型），这时myisam就自动使用动态型，虽然动态型的表占用了比静态型表较少的空间，但带来了性能的降低，因为如果某个字段的内容发生改变则其位置很可能需要移动，这样就会导致碎片的产生。随着数据变化的怎多，碎片就会增加，数据访问性能就会相应的降低。  对于因为碎片的原因而降低数据访问性，有两种解决办法：  
        1. 尽可能使用静态数据类型。  
        2. 经常使用optimize table语句，他会整理表的碎片，恢复由于表的更新和删除导致的空间丢失。(如果存储引擎不支持 optimize table 则可以转储并重新加载数据，这样也可以减少碎片。)
    3. MyISAM也是使用B+tree索引但是和InnoDB的在具体实现上有些不同。

### MEMORY ###
1. MEMORY存储引擎相比前面的一些存储引擎，有点不一样，其使用存储在内从中的数据来创建表，而且所有的数据也都存储在内存中。
2. 每个基于MEMORY存储引擎的表实际对应一个磁盘文件，该文件的文件名和表名是相同的，类型为.frm。该文件只存储表的结构，而其数据文件，都是存储在内存中，这样有利于对数据的快速处理，提高整个表的处理能力。
3. MEMORY存储引擎默认使用哈希（HASH）索引，其速度比使用B-+Tree型要快，如果希望使用B树型，则在创建的时候可以引用。
4. MEMORY存储引擎文件数据都存储在内存中，如果MYSQLD进程发生异常，重启或关闭机器这些数据都会消失。所以MEMORY存储引擎中的表的生命周期很短，一般只使用一次。

### 各引擎互相转换  ###
1. `ALTER TABLE tablename ENGINE = INnodb /MyISAM/Memory`。
    1. 优点是简单，适用于所有存储引擎。
    2. 缺点:
        1. 这种转化方式需要大量的时间 和I/O，MYSQL要执行从旧表 到新表的一行一行的复制所以效率比较低。
        2. 在转化这期间源表加了读锁。
        3. 从一种引擎到另一种引擎做表转化，所有属于原始引擎的专用特性都会丢失，比如从InnoDB到 MyISAM 则 InnoDB的索引会丢失！
2. 使用mysqldump，和import
