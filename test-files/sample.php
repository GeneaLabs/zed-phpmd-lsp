<?php

namespace TestFiles;

/**
 * Sample file with various PHPMD issues for testing
 */
class SampleClass
{
    // Unused private property (unusedcode ruleset)
    private $unusedProperty;
    
    // Short variable name (naming ruleset)
    private $x;
    
    // Another unused property
    private $neverUsedVariable = 'test';
    
    /**
     * Method with too many parameters (cleancode/codesize ruleset)
     */
    public function tooManyParameters($param1, $param2, $param3, $param4, $param5, $param6, $param7, $param8)
    {
        // Long variable name (naming ruleset)
        $thisIsAVeryLongVariableNameThatExceedsRecommendedLength = $param1;
        
        return $param2;
    }
    
    /**
     * Excessively complex method (codesize ruleset - cyclomatic complexity)
     */
    public function overyComplexMethod($input)
    {
        if ($input == 1) {
            if ($this->checkSomething()) {
                if ($this->checkAnotherThing()) {
                    if ($this->yetAnotherCheck()) {
                        if ($this->stillMoreChecks()) {
                            if ($this->tooManyNestingLevels()) {
                                return 'deeply nested';
                            }
                        }
                    }
                }
            }
        } elseif ($input == 2) {
            return 'two';
        } elseif ($input == 3) {
            return 'three';
        } elseif ($input == 4) {
            return 'four';
        } elseif ($input == 5) {
            return 'five';
        } elseif ($input == 6) {
            return 'six';
        } elseif ($input == 7) {
            return 'seven';
        } elseif ($input == 8) {
            return 'eight';
        } elseif ($input == 9) {
            return 'nine';
        } elseif ($input == 10) {
            return 'ten';
        }
        
        return 'default';
    }
    
    /**
     * Method that's never called (unusedcode ruleset)
     */
    private function unusedPrivateMethod()
    {
        return 'This method is never used';
    }
    
    /**
     * Method with code duplication
     */
    public function duplicatedCode1($value)
    {
        // This code block is duplicated
        $result = $value * 2;
        $result = $result + 10;
        $result = $result / 3;
        $result = round($result, 2);
        
        return $result;
    }
    
    /**
     * Another method with the same duplicated code
     */
    public function duplicatedCode2($value)
    {
        // This code block is duplicated
        $result = $value * 2;
        $result = $result + 10;
        $result = $result / 3;
        $result = round($result, 2);
        
        return $result;
    }
    
    /**
     * Empty catch block (cleancode ruleset)
     */
    public function emptyCatchBlock()
    {
        try {
            $this->doSomethingDangerous();
        } catch (\Exception $e) {
            // Empty catch block - bad practice
        }
    }
    
    /**
     * Method using global variables (controversial ruleset)
     */
    public function usingGlobals()
    {
        global $globalVar;
        return $globalVar;
    }
    
    /**
     * Method with boolean flag parameter (cleancode ruleset)
     */
    public function booleanArgumentFlag($data, $useSpecialLogic = false)
    {
        if ($useSpecialLogic) {
            return $this->processSpecial($data);
        }
        return $this->processNormal($data);
    }
    
    /**
     * Static access (cleancode ruleset - controversial)
     */
    public function staticAccess()
    {
        return StaticClass::doSomething();
    }
    
    /**
     * Overly long method (codesize ruleset)
     */
    public function veryLongMethod()
    {
        $step1 = 1;
        $step2 = 2;
        $step3 = 3;
        $step4 = 4;
        $step5 = 5;
        $step6 = 6;
        $step7 = 7;
        $step8 = 8;
        $step9 = 9;
        $step10 = 10;
        $step11 = 11;
        $step12 = 12;
        $step13 = 13;
        $step14 = 14;
        $step15 = 15;
        $step16 = 16;
        $step17 = 17;
        $step18 = 18;
        $step19 = 19;
        $step20 = 20;
        $step21 = 21;
        $step22 = 22;
        $step23 = 23;
        $step24 = 24;
        $step25 = 25;
        $step26 = 26;
        $step27 = 27;
        $step28 = 28;
        $step29 = 29;
        $step30 = 30;
        $step31 = 31;
        $step32 = 32;
        $step33 = 33;
        $step34 = 34;
        $step35 = 35;
        $step36 = 36;
        $step37 = 37;
        $step38 = 38;
        $step39 = 39;
        $step40 = 40;
        $step41 = 41;
        $step42 = 42;
        $step43 = 43;
        $step44 = 44;
        $step45 = 45;
        $step46 = 46;
        $step47 = 47;
        $step48 = 48;
        $step49 = 49;
        $step50 = 50;
        
        return $step1 + $step50;
    }
    
    /**
     * Exit expression (design ruleset)
     */
    public function usingExit($condition)
    {
        if ($condition) {
            exit('Stopping execution');
        }
        return true;
    }
    
    /**
     * Eval expression (design ruleset - security issue)
     */
    public function usingEval($code)
    {
        return eval($code);
    }
    
    /**
     * Goto statement (cleancode ruleset)
     */
    public function usingGoto($value)
    {
        if ($value < 0) {
            goto error;
        }
        
        return $value;
        
        error:
        return -1;
    }
    
    /**
     * Development/debug code left in (design ruleset)
     */
    public function debugCode($data)
    {
        var_dump($data); // Debug code
        print_r($data);  // More debug code
        
        return $data;
    }
    
    // Helper methods for testing
    private function checkSomething()
    {
        return true;
    }
    
    private function checkAnotherThing()
    {
        return true;
    }
    
    private function yetAnotherCheck()
    {
        return true;
    }
    
    private function stillMoreChecks()
    {
        return true;
    }
    
    private function tooManyNestingLevels()
    {
        return true;
    }
    
    private function doSomethingDangerous()
    {
        throw new \Exception('Dangerous operation');
    }
    
    private function processSpecial($data)
    {
        return $data;
    }
    
    private function processNormal($data)
    {
        return $data;
    }
}

/**
 * Class with too many fields (codesize ruleset)
 */
class TooManyFields
{
    private $field1;
    private $field2;
    private $field3;
    private $field4;
    private $field5;
    private $field6;
    private $field7;
    private $field8;
    private $field9;
    private $field10;
    private $field11;
    private $field12;
    private $field13;
    private $field14;
    private $field15;
    private $field16;
    private $field17;
    private $field18;
    private $field19;
    private $field20;
}

/**
 * Class with too many methods (codesize ruleset)
 */
class TooManyMethods
{
    public function method1() {}
    public function method2() {}
    public function method3() {}
    public function method4() {}
    public function method5() {}
    public function method6() {}
    public function method7() {}
    public function method8() {}
    public function method9() {}
    public function method10() {}
    public function method11() {}
    public function method12() {}
    public function method13() {}
    public function method14() {}
    public function method15() {}
    public function method16() {}
    public function method17() {}
    public function method18() {}
    public function method19() {}
    public function method20() {}
    public function method21() {}
    public function method22() {}
    public function method23() {}
    public function method24() {}
    public function method25() {}
    public function method26() {}
}

/**
 * Static class for testing static access
 */
class StaticClass
{
    public static function doSomething()
    {
        return 'static result';
    }
}