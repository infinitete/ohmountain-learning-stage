<?php

/**********************/
// 测试工厂模式
/**********************/

use Nineteen\Design\Pattern\Factory;
use Nineteen\Design\Pattern\Additional\Database;

class FactoryTest extends \PHPUnit\Framework\TestCase
{
    public function testFactory()
    {
        $this->assertTrue(Factory::getDatabase() instanceof Database);
    }
}