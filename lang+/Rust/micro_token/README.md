# micro_token Rust版 性能 #

## 功能描述和性能测试  ##

### generate API 功能 ###
通过GET的方式获取一个UUID键值对和它的TTL,返回格式是JSON，程序内部对Redis只有写入

### validate API 功能 ###
通过POST的方式验证一个UUID的键值对是否存在Redis里，返回格式是JSON，请求需要两个参数：key和val，分别是redis的键名和值，程序内部需要获取请求值，对Redis是只读

### generate API 性能测试  ###

```shell
This is ApacheBench, Version 2.3 <$Revision: 1706008 $>  
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/  
Licensed to The Apache Software Foundation, http://www.apache.org/  
  
Benchmarking localhost (be patient)  
  
  
Server Software:        mkd  
Server Hostname:        localhost  
Server Port:            3000  
  
Document Path:          /  
Document Length:        88 bytes  
  
Concurrency Level:      1000  
Time taken for tests:   0.381 seconds  
Complete requests:      10000  
Failed requests:        0  
Total transferred:      2290000 bytes  
HTML transferred:       880000 bytes  
Requests per second:    26227.93 [#/sec] (mean)  
Time per request:       38.127 [ms] (mean)  
Time per request:       0.038 [ms] (mean, across all concurrent requests)  
Transfer rate:          5865.42 [Kbytes/sec] received  
  
Connection Times (ms)  
              min  mean[+/-sd] median   max  
Connect:        0    2   1.4      2      11  
Processing:     2    5   3.0      4     199  
Waiting:        1    4   3.1      4     199  
Total:          3    6   3.3      6     199  
  
Percentage of the requests served within a certain time (ms)  
  50%      6  
  66%      6  
  75%      7  
  80%      7  
  90%      8  
  95%      9  
  98%     14  
  99%     15  
 100%    199 (longest request)  
```

### validate API 性能测试 ###
```shell
This is ApacheBench, Version 2.3 <$Revision: 1706008 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking localhost (be patient)


Server Software:        mkd
Server Hostname:        localhost
Server Port:            3000

Document Path:          /validate
Document Length:        19 bytes

Concurrency Level:      1000
Time taken for tests:   0.313 seconds
Complete requests:      10000
Failed requests:        0
Total transferred:      1600000 bytes
Total body sent:        1780000
HTML transferred:       190000 bytes
Requests per second:    31931.84 [#/sec] (mean)
Time per request:       31.317 [ms] (mean)
Time per request:       0.031 [ms] (mean, across all concurrent requests)
Transfer rate:          4989.35 [Kbytes/sec] received
                        5550.65 kb/s sent
                        10540.00 kb/s total

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        1    2   1.0      2       9
Processing:     1    4   1.1      3      17
Waiting:        1    3   1.0      3      16
Total:          3    6   1.6      6      19

Percentage of the requests served within a certain time (ms)
  50%      6
  66%      6
  75%      7
  80%      7
  90%      8
  95%      9
  98%     12
  99%     12
 100%     19 (longest request)
 ```
 
## 后记 ##
[Iron](https://github.com/iron/iron)是一个基于[Hyper](https://github.com/hyperium/hyper)的很优秀的Rust Web框架，性能卓越，Iron和很多现代Web框一样是一个组件化的框架。周末花了一天一夜使用Iron做了这个小小的程序；在写这个程序之前对Iron了解不多，最多就是Hello World的水平；通过一天一夜的奋战，终于可以开发出一个简单的Web程序，这过程中，学习了很多Iron和Hyper相关的知识，如Request、Response、Headers、Body等。  
较大的一坑是由于没有详细看Iron的另外一个库[iron/params](https://github.com/iron/params)，导致写validate API的时候通过读取Request Body，然后使用正则匹配的方式获取请求参数，导致性能下降90%(哎哟我去，一度怀疑是不是使用太多String Clone造成的，后来确认是正则库[regex](https://github.com/rust-lang/regex)的性能实在是太差，不过也可能是读取Request Body的锅)，使用iron/params后，性能马上恢复正常。  
与PHP版本相比([另一个PHP版本的的实现](https://github.com/ohmountain/ohmountain-learning-stage/tree/master/lang%2B/PHP/micro_token))，PHP版本的性能是Rust版本性能的20%以下，Rust版的响应时间都是十多毫秒，而PHP版本则达1秒左右。相较开发难度而言，Rust无疑是没有任何优势的，我可以在半小时内完成PHP版本，但完成Rust版本却需要一天一夜。
