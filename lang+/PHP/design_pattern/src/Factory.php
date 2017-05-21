<?php

/****************************************/
// PHP 设计模式 之 工厂模式
/****************************************/

namespace Nineteen\Design\Pattern;

use Nineteen\Design\Pattern\Additional\Database;

class Factory
{
    public static function getDatabase()
    {
        return new Database();
    }
}