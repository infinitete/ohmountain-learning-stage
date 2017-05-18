<?php

/****************************************/
// PHP 设计模式 之 适配器模式
/****************************************/

namespace Nineteen\Design\Pattern\Adapter;

interface DbInterface
{
    public function connection(string $host, string $port, string $user, string $pass, string $db);
    public function query(string $sql);
    public function close();
}