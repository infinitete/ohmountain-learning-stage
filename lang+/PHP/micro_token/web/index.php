<?php

require_once(__DIR__.'/../vendor/autoload.php');

use Nineteen\Redis\Runner;

header("content-type: application/json; charset=utf-8");

echo (new Runner())->run();