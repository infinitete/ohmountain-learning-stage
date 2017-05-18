<?php

/****************************************/
// PHP 设计模式 之 适配器模式
/****************************************/

namespace Nineteen\Design\Pattern\Adapter;

class MySQLiAdapter implements DbInterface
{

    /**
     * @var mysqli $connection
     */
    private $connection;

    /**
     * @param string $host
     * @param string $port
     * @param string $user
     * @param string $pass
     * @param string $db
     *
     * @throw \Exception
     */
    public function connection(string $host, string $port, string $user, string $pass, string $db)
    {
        $mysqli = new \mysqli($host, $user, $pass, $db, $port);

        $this->connection = $mysqli;
    }

    /**
     * param string $sql
     */
    public function query(string $sql)
    {
        return $this->connection->query($sql);
    }

    public function close()
    {
        mysqli_close($this->mysqli);
    }
}