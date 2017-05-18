<?php

/****************************************/
// PHP 设计模式 之 适配器模式
/****************************************/

namespace Nineteen\Design\Pattern\Adapter;

class PdoAdapter implements DbInterface
{
    private $connection;

    /**
     * @param string $host
     * @param string $port
     * @param string $user
     * @param string $pass
     * @param string $db
     */
    public function connection(string $host, string $port, string $user, string $pass, string $db)
    {
        $dsn = "mysql:host={$host};port={$port};dbname={$db}";

        $this->connection = new PDO($dsn);
    }

    /**
     * @param string $sql
     *
     * @throw \PDOException
     *
     * @return \PDOStatement
     */
    public function query(string $sql): \PDOStatement
    {
       return $this->connection->query($sql);
    }

    public function close()
    {
        unset($this->connection);
    }
}