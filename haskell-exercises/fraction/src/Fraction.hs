module Fraction (Fraction, add, sub, mul, divide, hcf) where

type Fraction = (Int, Int)

-- Implement the `add` Function

add :: Fraction -> Fraction -> Fraction
add (numA, denA) (numB, denB)
  | denA == 0 || denB == 0 = error "Fractions can't be divided by 0"
  | denA == denB = (numA + numB, denA)
  | otherwise = result
    where
      newNum = numA * denB + numB * denA
      newDen = denA * denB
--      result = (newNum, newDen)
      result = simplify (newNum, newDen)

-- Implement the `sub` Function

sub :: Fraction -> Fraction -> Fraction
sub (numA, denA) (numB, denB)
  | denA == 0 || denB == 0 = error "Fractions can't be divided by 0"
  | denA == denB = (numA - numB, denA)
  | otherwise = result
    where
      newNum = numA * denB - numB * denA
      newDen = denA * denB
--      result = (newNum, newDen)
      result = simplify (newNum, newDen)

-- Implement the `mul` Function

mul :: Fraction -> Fraction -> Fraction
mul (numA, denA) (numB, denB)
  | denA == 0 || denB == 0 = error "Fractions can't be divided by 0"
  | otherwise = result
    where
      newNum = numA * numB
      newDen = denA * denB
--      result = (newNum, newDen)
      result = simplify (newNum, newDen)

-- Implement the `divide` Function

divide :: Fraction -> Fraction -> Fraction
divide (numA, denA) (numB, denB) = result
  where
    fraction = mul (numA, denA) (denB, numB)
--    result = fraction
    result = simplify fraction

-- Implement the `hcf` Function

hcf :: Int -> Int -> Int
hcf n d
  | d == 0 = n
  | n == 0 = d
  | otherwise = hcf smaller remainder
    where
      (greater, smaller) = getGreaterSmaller n d
      remainder = greater `rem` smaller

getGreaterSmaller :: Int -> Int -> (Int, Int)
getGreaterSmaller a b = if a > b then (a, b) else (b, a)

simplify :: Fraction -> Fraction
simplify (num, den) = result
  where
    factor = hcf num den
    newNum = num `div` factor
    newDen = den `div` factor
    result = (newNum, newDen)

