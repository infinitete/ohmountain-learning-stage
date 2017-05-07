# micro_token Rust版 性能 #

```shell
his is ApacheBench, Version 2.3 <$Revision: 1706008 $>  
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

## 检验API性能问题 ##
1. 已经确定性能问题不是操作Redis造成的，而是因为使用正则库匹配请求参数造成的
2. 性能下降严重，不使用正则可处理每秒3w7请求左右，使用正则以后每秒只能处理5k请求
3. 难道这就是hyper或者iron没有提供任何获取请求参数的原因？
