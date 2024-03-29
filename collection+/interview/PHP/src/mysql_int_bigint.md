# MySQL中int和bigint的区别 #

# 答案 #
在MySQL中，整型有不同的表示，根据使用情况来集体使用:  

1. tinyint  
tinyint 使用一个字节来保存数据，一个字节即8位，存储范围为-128 - 127,如果是无符号(unsigned)，存储范围则为0-255.
2. smallint  
smallint 使用两个字节来存储，两个字节即16位，存储范围为-2^15 - 2^15-1,即-32768 - 32767，如果是无符号(unsigned)，存储范围则为0-65535.
3. mediumint  
mediumint 使用三个字节来存储，三个字节为24位，存储范围为-2^23 - 2^23-1,即-8388608 - 8388607，如果是无符号(unsigned)，存储范围则为0-16777215.
4. int  
int 使用四个来存储，四个字节为32位，存储范围为-2^31 - 2^31-1,即-2147483648 - 2147483648，如果是无符号(unsigned)，存储范围则为0-4294967295.
5. bigint  
bigint 使用8个字节来存储，8个字节为64位，存储范围为-2^63 - 2^63-1,即-9223372036854775808 - 9223372036854775807, 如果是无符号则为0-18446744073709551615。

取值范围如下表：  

| 类型      |  存储大小 |               最小值 |               最大值 |
| ----      | :-------: |      :-------------: |      :-------------: |
|           |   (bytes) |    (Singed/Unsigned) |    (Singed/Unsigned) |
| TINYINT   |         1 |                 -128 |                  127 |
|           |           |                    0 |                  255 |
| SMALLINT  |         2 |               -32768 |                32767 |
|           |           |                    0 |                65535 |
| MEDIUMINT |         3 |             -8388608 |              8388607 |
|           |           |                    0 |             16777215 |
| INT       |         4 |          -2147483648 |           2147483647 |
|           |           |                    0 |           4294967295 |
| BIGINT    |         8 | -9223372036854775808 |  9223372036854775807 |
|           |           |                    0 | 18446744073709551615 |

> 来源：[MySQL](https://dev.mysql.com/doc/refman/5.7/en/integer-types.html)

在数据库实际应用中，可视具体情形来选择类型。如作为主键，可直接适应INT,4294967295将近43亿条记录是很难想象的。如果一个字段使用的是INT类型，如INT(10)，而某个真实的值是100,那么它将被填充7个0,变成0000000100，而如果是INT(11),那么将被填充8个0,变成00000000100；实际上，INT的最大值是4294967296，INT(10)已经超出了存储范围，长度太大只会增加MySQL负担。
