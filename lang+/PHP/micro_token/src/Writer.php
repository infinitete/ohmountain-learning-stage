<?php

namespace Nineteen\Redis;

class Writer
{
    private $redis;

    public function __construct()
    {
        $redis = new \Redis();
        $redis->connect("127.0.0.7", 6379);

        $this->redis = $redis;
    }

    public function set($key, $val, $ttl = 60)
    {
        $this->redis->set($key, $val);
        $this->redis->expire($key, $ttl);
    }
}