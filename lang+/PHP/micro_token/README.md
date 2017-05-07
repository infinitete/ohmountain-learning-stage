# micro_token PHP 版 性能 #

```shell

This is ApacheBench, Version 2.3 <$Revision: 1706008 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking localhost (be patient)


Server Software:          
Server Hostname:        localhost  
Server Port:            8080  

Document Path:          /  
Document Length:        88 bytes  

Concurrency Level:      1000  
Time taken for tests:   3.221 seconds  
Complete requests:      10000  
Failed requests:        0  
Total transferred:      2380000 bytes  
HTML transferred:       880000 bytes  
Requests per second:    3104.57 [#/sec] (mean)  
Time per request:       322.105 [ms] (mean)  
Time per request:       0.322 [ms] (mean, across all concurrent requests)  
Transfer rate:          721.57 [Kbytes/sec] received  

Connection Times (ms)  
              min  mean[+/-sd] median   max  
Connect:        0    5  68.3      0    1000  
Processing:     4   38 111.6     27    3205  
Waiting:        4   38 111.6     27    3205  
Total:          9   43 173.3     27    3205  

Percentage of the requests served within a certain time (ms)  
  50%     27  
  66%     28  
  75%     29  
  80%     30  
  90%     31  
  95%     56  
  98%    122  
  99%    144  
 100%   3205 (longest request)  

```
