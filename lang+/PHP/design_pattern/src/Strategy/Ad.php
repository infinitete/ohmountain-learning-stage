<?php

/****************************************/
// PHP 设计模式 之 策略模式
/****************************************/

namespace Nineteen\Design\Pattern\Strategy;

use Nineteen\Design\Pattern\Strategy\Specs\AdInterface;

abstract class Ad implements AdInterface
{
    private $word;

    public function init(string $word)
    {
        $this->word = $word;
    }

    public function show(): string
    {
        return $this->word;
    }

}