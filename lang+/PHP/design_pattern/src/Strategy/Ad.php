<?php

/****************************************/
// PHP 设计模式 之 策略模式
/****************************************/

namespace Nineteen\Design\Pattern\Strategy;

use Nineteen\Design\Pattern\Strategy\Specs\AdInterface;

// 广告策略的实现，每一个广告斗必须遵循这个实现
// 只要遵循广告接口，任何广告类可以扩展这个实现

abstract class Ad implements AdInterface
{
    private $word;

    /**
     * @param string $word
     */
    public function init(string $word)
    {
        $this->word = $word;
    }

    /**
     * @return string
     */
    public function show(): string
    {
        return $this->word;
    }

}