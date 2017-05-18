<?php

/**********************/
// 测试适配器模式
/**********************/

use Nineteen\Design\Pattern\Adapter\MySQLiAdapter;

class MySQLiTest extends PHPUnit\Framework\TestCase
{
    private $host = "localhost";
    private $port = "3306";
    private $user = "root";
    private $pass = "trienshan025";
    private $db   = "test";

    private $mysqli = null;

    /**
     * @before
     */
    public function before()
    {
        $mysqli = new MySQLiAdapter();

        $this->mysqli = $mysqli->connection($this->host, $this->port, $this->user, $this->pass, $this->db);
    }
}