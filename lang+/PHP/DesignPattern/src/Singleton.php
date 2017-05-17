<?php

/****************************************/
// PHP 设计模式 之  单例模式
/****************************************/

namespace Nineteen\Design\Pattern;

class Singleton
{
    private static $instance;

    private function __construct()
    {
    }

    public static function getInstance()
    {
        if (self::$instance instanceof self) {
            return self::$instance;
        }

        self::$instance = new self();

        return self::$instance;
    }
}