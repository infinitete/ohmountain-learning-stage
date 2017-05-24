<?php

/****************************************/
// PHP 设计模式 之 策略模式
/****************************************/

namespace Nineteen\Design\Pattern\Strategy;

class FemaleAd extends Ad
{
    public function show(): string
    {
        $word = parent::show();

        return "女性广告： $word";
    }
}