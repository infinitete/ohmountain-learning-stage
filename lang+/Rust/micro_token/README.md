# micro_token Rust版 性能 #

### generate API  ###

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

### validate API ###
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
