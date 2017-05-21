<?php

/**********************/
// 测试注册器模式
/**********************/

use Nineteen\Design\Pattern\Register;
use Nineteen\Design\Pattern\Additional\Database;

class RegisterTest extends \PHPUnit\Framework\TestCase
{
    private $register;

    private $names = ['db1', 'db2'];

    /**
     * @before
     */
    public function before()
    {
        $this->register = new Register();

        $this->register->register($this->names[0], 'Nineteen\Design\Pattern\Additional\Database');
        $this->register->register($this->names[1], 'Nineteen\Design\Pattern\Additional\Database');
    }

    public function testRegister()
    {
        foreach ($this->names as $name) {
            $this->assertTrue($this->register->has($name));
        }
    }

    /**
     * @expectedException          \Exception
     * @expectedExceptionMessage   Fatal: name db1 has been registered
     */
    public function testRegisterTwice()
    {
        $this->register->register($this->names[0], 'Nineteen\Design\Pattern\Additional\Database');
    }

    /**
     * @expectedException          \Exception
     * @expectedExceptionMessage   Fatal: class Not\AClass dosn't exists
     */
    public function testRegisterANonExistsClass()
    {
        $this->register->register('some_none_exists_class', 'Not\AClass');
    }

    public function testGet()
    {
        $this->assertTrue($this->register->get($this->names[0]) instanceof Database);
        $this->assertTrue($this->register->get($this->names[1]) instanceof Database);

        $this->assertTrue($this->register->get('Not\AClass') === null);
    }

    /**
     * 测试返回值是不是单例的
     */
    public function testGetWithSingleton()
    {
        $this->assertSame($this->register->get($this->names[0]), $this->register->get($this->names[0]));
        $this->assertSame($this->register->get($this->names[1]), $this->register->get($this->names[1]));
    }

    public function testRaw()
    {
        $r00 = $this->register->raw($this->names[0]);
        $r01 = $this->register->raw($this->names[0]);

        $this->assertSame(get_parent_class($r00), get_parent_class($r01));
        $this->assertNotSame($r00, $r01);


        $r10 = $this->register->raw($this->names[1]);
        $r11 = $this->register->raw($this->names[1]);

        $this->assertSame(get_parent_class($r10), get_parent_class($r11));
        $this->assertNotSame($r10, $r11);
    }

    public function testRomove()
    {
        $this->assertTrue($this->register->remove($this->names[0]));

        $this->assertNull($this->register->get($this->names[0]));
        $this->assertNull($this->register->raw($this->names[0]));
    }

    public function testHas()
    {
        $this->assertTrue($this->register->has($this->names[0]));
        $this->assertTrue($this->register->has($this->names[1]));

        $this->assertFalse($this->register->has('Not\AClass'));
    }
}