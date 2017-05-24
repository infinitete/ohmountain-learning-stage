<?php

/****************************************/
// PHP 设计模式 之 策略模式
/****************************************/

namespace Nineteen\Design\Pattern\Strategy;

// 为男性用户显示的广告
// 这个广告遵循了规范

class MaleAd extends Ad
{
    /**
     * @return string
     */
    public function show(): string
    {
        $word = parent::show();

        return "男性广告： $word";
    }
}