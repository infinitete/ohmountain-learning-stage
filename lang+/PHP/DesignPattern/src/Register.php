<?php

/****************************************/
// PHP 设计模式 之 注册器模式 
/****************************************/

namespace Nineteen\Design\Pattern;

class Register
{
    private $maps;
    private $instances;

    /**
     * 不可继承
     */
    public final function __construct()
    {
        $this->maps      = array();
        $this->instances = array();
    }

    /**
     * 注册，并不是注册后马上就实例化
     *
     * @param string $name    别名
     * @param string $class   映射类
     *
     * @throw \Exception
     */
    public function register(string $name, string $class)
    {
        if (array_key_exists($name, $this->maps)) {
            throw new \Exception("Fatal: name {$name} has been registered");
        }

        if (!class_exists($class)) {
            throw new \Exception("Fatal: class {$class} dosn't exists");
        }

        $this->maps[$name] = $class;
    }

    /**
     * 获取一个实例化的对象
     *
     * @param string $name
     */
    public function get(string $name)
    {
        if (!array_key_exists($name, $this->maps)) {
            return null;
        }

        if (!array_key_exists($name, $this->instances)) {
            $this->instances[$name] = new $this->maps[$name]();
        }

        return $this->instances[$name];
    }

    /**
     * 获取一个新的实例对象，而且这个对象不会被挂载到注册器上
     *
     * @param string $name
     */
    public function raw(string $name)
    {
        if (!array_key_exists($name, $this->maps)) {
            return null;
        }

        return new $this->maps[$name]();
    }

    /**
     * 从注册器上移除一个映射
     *
     * @param string $name
     *
     * @return bool
     */
    public function remove(string $name): bool
    {
        if (!array_key_exists($name, $this->maps)) {
            return true;
        }

        unset($this->maps[$name]);

        if (!array_key_exists($name, $this->instances)) {
            return true;
        }

        unset($this->instances[$name]);
    }

    /**
     * 检测是否已被挂载
     *
     * @param string $name
     *
     * @return bool
     */
    public function has(string $name): bool
    {
        return array_key_exists($name, $this->maps);
    }
}
