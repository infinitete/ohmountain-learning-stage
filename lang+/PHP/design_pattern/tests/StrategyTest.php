<?php

/**********************/
// 测试策略模式
/**********************/

use Nineteen\Design\Pattern\Strategy\{ MaleAd, FemaleAd };

class StrategyTest extends \PHPUnit\Framework\TestCase
{
    private $male_prefix  = '男性广告： ';
    private $male_word    = '来吧，这里有很多最新的电子产品';

    private $female_prefix = '女性广告： ';
    private $female_word   = 'Baby，最新的美人裙到货啦';

    public function testMale()
    {
        $ad = new MaleAd();
        $ad->init($this->male_word);

        $this->assertEquals($this->male_prefix . $this->male_word, $ad->show());
    }

    public function testFemale()
    {
        $ad = new FemaleAd();
        $ad->init($this->female_word);

        $this->assertEquals($this->female_prefix . $this->female_word, $ad->show());
    }

    public function testSameParent()
    {
        $m_ad = new MaleAd();
        $f_ad = new FemaleAd();

        $this->assertSame(get_parent_class($m_ad), get_parent_class($f_ad));
    }
}