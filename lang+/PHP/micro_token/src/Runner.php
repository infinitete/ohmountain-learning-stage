<?php

namespace Nineteen\Redis;

use Ramsey\Uuid\Uuid;
use Ramsey\Uuid\Exception\UnsatisfiedDependencyException;

class Runner
{
    private $writer;

    public function __construct()
    {
        $this->writer = new Writer();
    }

    public function run()
    {
        try {
            $key = Uuid::uuid4();
            $val = Uuid::uuid4();

            $ttl = 60;

            $this->writer->set($key->toString(), $val->toString(), $ttl);

            return json_encode([
                $key->toString()  => $val->toString(),
                "ttl" => $ttl
            ]);
        } catch (\Exception $e) {
        }

        return "";
    }
}