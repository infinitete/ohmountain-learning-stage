<?php

/**********************/
// 测试单例模式
/**********************/

use Nineteen\Design\Pattern\Singleton;

class SingletonTest extends \PHPUnit\Framework\TestCase
{
    public function testSingleton()
    {
        $s1 = Singleton::getInstance();
        $s2 = Singleton::getInstance();
        $s3 = Singleton::getInstance();

        $this->assertEquals($s1, $s2);
        $this->assertEquals($s1, $s3);
        $this->assertEquals($s2, $s3);
    }
}