<?php

/****************************************/
// PHP 设计模式 之 策略模式
/****************************************/

namespace Nineteen\Design\Pattern\Strategy\Specs;

// 这是一个广告规范，
// 这个规范会衍生不同的具体广告类，
// 遵循这个规范的广告类将被接受

interface AdInterface
{
    public function init(string $word);
    public function show(): string;
}
